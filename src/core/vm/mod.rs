use crate::core::compiler::parser::tree::Type;
use crate::core::shared::bytecode::{Instruction, Opcode};
use crate::core::shared::executable::Executable;
use crate::core::vm::error::VmError;
use crate::core::vm::memory::{Memory, Value};
use crate::core::vm::number::Number;

pub mod debug;
pub mod error;
pub mod memory;
pub mod number;
pub mod operators;

pub struct VirtualMachine {
    memory: Memory,
    pc: usize,
    instructions: Vec<Instruction>,
    halted: bool,
    debug: bool,
}

impl VirtualMachine {
    pub fn new(heap_size: usize) -> Self {
        Self {
            memory: Memory::new(heap_size),
            pc: 0,
            instructions: Vec::new(),
            halted: false,
            debug: false,
        }
    }

    /// Copies data into heap and sets instructions and entry point.
    pub fn load_executable(&mut self, executable: &Executable) -> Result<(), VmError> {
        let data = &executable.data;
        let mut offset = 0usize;

        // For each item: copy it int heap
        while offset < data.len() {
            if offset + 1 + 4 > data.len() {
                return Err(VmError::InvalidExecutable);
            }

            let rtti_len = data[offset] as usize;
            let rtti_start = offset + 1;
            if rtti_start + rtti_len + 4 > data.len() {
                return Err(VmError::InvalidExecutable);
            }

            let data_len_start = rtti_start + rtti_len;
            let data_len = u32::from_le_bytes([
                data[data_len_start],
                data[data_len_start + 1],
                data[data_len_start + 2],
                data[data_len_start + 3],
            ]) as usize;

            let total_size = 1 + rtti_len + 4 + data_len;
            if offset + total_size > data.len() {
                return Err(VmError::InvalidExecutable);
            }

            // Allocate a block for this single item
            let addr = self.memory.allocate(total_size, true)?;
            // Copy the record into heap
            self.memory
                .write_bytes(addr, &data[offset..offset + total_size])?;

            offset += total_size;
        }

        self.instructions = executable.instructions.clone();
        self.pc = executable.header.entry_point as usize;
        Ok(())
    }

    /// Execute until halted
    pub fn run(&mut self) -> Result<(), VmError> {
        self.memory.push_frame(0);
        while !self.halted && self.pc < self.instructions.len() {
            self.step()?;
        }
        self.memory.collect_garbage()?; // Collect garbage on exit
        Ok(())
    }

    /// Execute a single instruction
    pub fn step(&mut self) -> Result<(), VmError> {
        if self.halted || self.pc >= self.instructions.len() {
            return Ok(());
        }

        let instr = self.instructions[self.pc];
        self.pc = self.pc.saturating_add(1);

        self.execute_instruction(instr)
    }

    fn execute_instruction(&mut self, instr: Instruction) -> Result<(), VmError> {
        match instr.opcode {
            // Stack
            Opcode::PUSH_IMM => {
                let value = instr.operand;
                self.push_num(Number::Int(value))?;
            }
            Opcode::PUSH_FLOAT_IMM => {
                let value = Instruction::bitcast_int(instr.operand);
                self.push_num(Number::Float(value))?;
            }
            Opcode::PUSH_HEAP_REF => {
                let addr = instr.operand as usize;
                self.push_ref(addr)?;
            }
            Opcode::PUSH_LOCAL_REF => {
                let idx = instr.operand as usize;
                let value = self.memory.get_local(idx)?;
                self.memory.push(value)?;
            }
            Opcode::POP => {
                self.memory.pop()?;
            }
            Opcode::DUP => {
                let value = self.peek_val()?;
                self.memory.push(value)?;
            }
            Opcode::SWAP => {
                let a = self.memory.pop()?;
                let b = self.memory.pop()?;
                self.memory.push(a)?;
                self.memory.push(b)?;
            }
            Opcode::ROT => {
                let c = self.memory.pop()?;
                let b = self.memory.pop()?;
                let a = self.memory.pop()?;
                self.memory.push(c)?;
                self.memory.push(a)?;
                self.memory.push(b)?;
            }

            // Arithmetic
            Opcode::ADD => self.op_add()?,
            Opcode::SUB => self.op_sub()?,
            Opcode::MUL => self.op_mul()?,
            Opcode::DIV => self.op_div()?,
            Opcode::REM => self.op_rem()?,
            Opcode::POW => self.op_pow()?,
            Opcode::NEG => self.op_neg()?,

            // Bitwise / logical
            Opcode::AND => self.op_and()?,
            Opcode::OR => self.op_or()?,
            Opcode::XOR => self.op_xor()?,
            Opcode::NOT => self.op_not()?,
            Opcode::SLA => self.op_sla()?,
            Opcode::SRA => self.op_sra()?,

            // Comparison (returns 1 or 0)
            Opcode::EQ | Opcode::NE | Opcode::LT | Opcode::GT | Opcode::LE | Opcode::GE => {
                self.compare_generic(instr.opcode)?
            }

            // Memory operations
            Opcode::LOAD => {
                self.op_load()?;
            }
            Opcode::STORE => {
                self.op_store()?;
            }
            Opcode::LOAD_LOCAL => {
                let idx = instr.operand as usize;
                let value = self.memory.get_local(idx)?;
                self.memory.push(value)?;
            }
            Opcode::STORE_LOCAL => {
                let idx = instr.operand as usize;
                let value = self.memory.pop()?;
                self.memory.set_local(idx, value)?;
            }

            // Control flow
            Opcode::JMP => {
                let target = instr.operand as usize;
                self.pc = target;
            }
            Opcode::JMPT => {
                let target = instr.operand as usize;
                let cond = self.pop_num()?.to_i32();
                if cond != 0 {
                    self.pc = target;
                }
            }
            Opcode::JMPF => {
                let target = instr.operand as usize;
                let cond = self.pop_num()?.to_i32();
                if cond == 0 {
                    self.pc = target;
                }
            }
            Opcode::CALL => {
                let target = instr.operand as usize;
                self.memory.push_frame(self.pc);
                self.pc = target;
            }
            Opcode::RET => {
                match self.memory.pop_frame() {
                    Ok(frame) => {
                        // Return address 0 signals program end
                        if frame.return_address == 0
                            || frame.return_address >= self.instructions.len()
                        {
                            self.halted = true;
                        } else {
                            self.pc = frame.return_address;
                        }
                    }
                    // No active frame so treat as program end
                    Err(_) => {
                        self.halted = true;
                    }
                }
            }

            // Misc
            Opcode::NOP => {}
            Opcode::HALT => {
                self.halted = true;
            }
            Opcode::SYSCALL => {
                self.op_syscall(instr.operand as u8)?;
            }
        }

        Ok(())
    }

    /// Parse heap RTTI into Type and return data slice
    fn heap_get_type_and_data(&self, addr: usize) -> Result<(Type, &[u8]), VmError> {
        let (rtti, data) = self.memory.load_from_heap(addr)?;
        match Type::from_bytes(rtti).map_err(|e| VmError::type_mismatch("valid rtti", e)) {
            Ok((ty, _)) => Ok((ty, data)),
            Err(err) => Err(err),
        }
    }

    /// Parse heap RTTI into Type
    fn heap_get_type(&self, addr: usize) -> Result<Type, VmError> {
        let rtti = self.memory.load_type_from_heap(addr)?;
        match Type::from_bytes(rtti).map_err(|e| VmError::type_mismatch("valid rtti", e)) {
            Ok((ty, _)) => Ok(ty),
            Err(err) => Err(err),
        }
    }

    /// Require an integer value (Value::Int or ref to Integer)
    pub fn require_int_value(&mut self, value: Value) -> Result<i32, VmError> {
        match self.value_to_num(value)? {
            Number::Int(i) => Ok(i),
            Number::Float(f) => Err(VmError::type_mismatch("integer", format!("{}", f))),
        }
    }

    /// Convert a stack Value or reference into a numeric Number (int or float)
    fn value_to_num(&mut self, value: Value) -> Result<Number, VmError> {
        match value {
            Value::Imm(i) => Ok(i),
            Value::Ref(addr) => {
                let (ty, data) = self.heap_get_type_and_data(addr)?;
                match ty {
                    Type::Integer => {
                        let bytes = self.extract_4_bytes(data, addr)?;
                        Ok(Number::Int(i32::from_le_bytes(bytes)))
                    }
                    Type::Float => {
                        let bytes = self.extract_4_bytes(data, addr)?;
                        Ok(Number::Float(f32::from_le_bytes(bytes)))
                    }
                    _ => Err(VmError::type_mismatch(
                        "numeric",
                        format!("ref(0x{:x})", addr),
                    )),
                }
            }
        }
    }

    /// Extract 4 bytes from data
    fn extract_4_bytes(&self, data: &[u8], addr: usize) -> Result<[u8; 4], VmError> {
        if data.len() < 4 {
            return Err(VmError::type_mismatch(
                "4 bytes",
                format!("small data at 0x{:x}", addr),
            ));
        }
        let mut arr = [0u8; 4];
        arr.copy_from_slice(&data[0..4]);
        Ok(arr)
    }

    /// Convert a stack Value into a string
    fn value_to_string(&mut self, value: &Value) -> Result<String, VmError> {
        match value {
            Value::Ref(addr) => {
                let (ty, data) = self.heap_get_type_and_data(*addr)?;
                if ty == Type::String {
                    Ok(std::str::from_utf8(data).unwrap_or_default().to_string())
                } else {
                    match self.value_to_num(Value::Ref(*addr))? {
                        Number::Int(i) => Ok(i.to_string()),
                        Number::Float(f) => Ok(f.to_string()),
                    }
                }
            }
            Value::Imm(i) => Ok(i.to_string()),
        }
    }

    /// Build data for heap
    /// Returns: RTTI bytes and Data bytes
    fn build_data<T: AsRef<[u8]>>(
        &self,
        data: T,
        rtti: Type,
    ) -> Result<(Vec<u8>, Vec<u8>), VmError> {
        let bytes = data.as_ref();
        let rtti_bytes = rtti.to_bytes();

        Ok((rtti_bytes.to_vec(), bytes.to_vec()))
    }

    fn push_num(&mut self, n: Number) -> Result<(), VmError> {
        self.memory.push(Value::Imm(n))
    }

    fn pop_num(&mut self) -> Result<Number, VmError> {
        let val = self.memory.pop()?;
        let num = val.as_num()?;
        Ok(num)
    }

    fn push_ref(&mut self, addr: usize) -> Result<(), VmError> {
        self.memory.push(Value::Ref(addr))
    }

    fn pop_ref(&mut self) -> Result<usize, VmError> {
        let val = self.memory.pop()?;
        let addr = val.as_ref()?;
        Ok(addr)
    }

    fn peek_val(&self) -> Result<Value, VmError> {
        self.memory.peek()
    }
}
