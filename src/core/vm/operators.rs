use crate::core::compiler::parser::tree::Type;
use crate::core::shared::builtin_func::SysCallId;
use crate::core::shared::bytecode::Opcode;
use crate::core::vm::error::VmError;
use crate::core::vm::memory::Value;
use crate::core::vm::{Number, VirtualMachine};
use std::io;
use std::io::Write;

/// Macro for binary bitwise operations (AND, OR, XOR)
macro_rules! bitwise_binary_op {
    ($name:ident, $op:expr) => {
        pub fn $name(&mut self) -> Result<(), VmError> {
            let b = self.memory.pop()?;
            let a = self.memory.pop()?;
            let bi = self.require_int_value(b)?;
            let ai = self.require_int_value(a)?;
            self.push_num(Number::Int($op(ai, bi)))
        }
    };
}

/// Macro for shift operations (left shift, right shift)
macro_rules! shift_op {
    ($name:ident, $shift_fn:expr) => {
        pub fn $name(&mut self) -> Result<(), VmError> {
            let b = self.memory.pop()?;
            let a = self.memory.pop()?;
            let bi = self.require_int_value(b)? as u32;
            let ai = self.require_int_value(a)?;
            self.push_num(Number::Int($shift_fn(ai, bi)))
        }
    };
}

impl VirtualMachine {
    // Bitwise binary operations
    bitwise_binary_op!(op_and, |a, b| a & b);
    bitwise_binary_op!(op_or, |a, b| a | b);
    bitwise_binary_op!(op_xor, |a, b| a ^ b);

    // Shift operations
    shift_op!(op_sla, |a: i32, b: u32| a.wrapping_shl(b));
    shift_op!(op_sra, |a: i32, b: u32| (a as u32).wrapping_shr(b) as i32);

    /// Numeric add or string concat (string if either operand is string)
    pub fn op_add(&mut self) -> Result<(), VmError> {
        let vb = self.memory.pop()?;
        let va = self.memory.pop()?;

        // Try numeric operation first
        match (self.value_to_num(va), self.value_to_num(vb)) {
            (Ok(na), Ok(nb)) => {
                let result = na.add(nb);
                self.push_num(result)?;
                return Ok(());
            }
            _ => {
                // Fall back to string concatenation
                let sa = self.value_to_string(&va)?;
                let sb = self.value_to_string(&vb)?;

                let (rtti, data) = self.concat_string(&sa, &sb)?;
                let addr = self.memory.save_to_heap(&rtti, &data, false)?;
                self.push_ref(addr)?;
            }
        }

        Ok(())
    }

    fn concat_string(&self, sa: &str, sb: &str) -> Result<(Vec<u8>, Vec<u8>), VmError> {
        let mut combined = Vec::with_capacity(sa.len() + sb.len());
        combined.extend_from_slice(sa.as_bytes());
        combined.extend_from_slice(sb.as_bytes());
        self.build_data(combined, Type::String)
    }

    pub fn op_sub(&mut self) -> Result<(), VmError> {
        let vb = self.memory.pop()?;
        let va = self.memory.pop()?;

        let result = va.as_num()?.subtract(vb.as_num()?);
        self.push_num(result)?;
        Ok(())
    }

    pub fn op_mul(&mut self) -> Result<(), VmError> {
        let vb = self.memory.pop()?;
        let va = self.memory.pop()?;

        let result = va.as_num()?.multiply(vb.as_num()?);
        self.push_num(result)?;
        Ok(())
    }

    pub fn op_div(&mut self) -> Result<(), VmError> {
        let vb = self.memory.pop()?;
        let va = self.memory.pop()?;

        // Check divisor
        self.check_divisor(va)?;
        self.check_divisor(vb)?;

        let result = va.as_num()?.divide(vb.as_num()?);
        self.push_num(result)?;
        Ok(())
    }

    pub fn op_rem(&mut self) -> Result<(), VmError> {
        let vb = self.memory.pop()?;
        let va = self.memory.pop()?;

        // Check divisor
        self.check_divisor(va)?;
        self.check_divisor(vb)?;

        let result = va.as_num()?.modulo(vb.as_num()?);
        self.push_num(result)?;
        Ok(())
    }

    fn check_divisor(&mut self, value: Value) -> Result<(), VmError> {
        if let Ok(value) = self.value_to_num(value) {
            match value {
                Number::Int(0) => Err(VmError::type_mismatch("non-zero", "divisor")),
                Number::Float(0.0) => Err(VmError::type_mismatch("non-zero", "divisor")),
                _ => Ok(()),
            }
        } else {
            Err(VmError::type_mismatch("numeric", "divisor"))
        }
    }

    pub fn op_pow(&mut self) -> Result<(), VmError> {
        let vb = self.memory.pop()?;
        let va = self.memory.pop()?;

        let result = va.as_num()?.pow(vb.as_num()?);
        self.push_num(result)?;
        Ok(())
    }

    pub fn op_neg(&mut self) -> Result<(), VmError> {
        let va = self.memory.pop()?;

        let result = va.as_num()?.negate();
        self.push_num(result)?;
        Ok(())
    }

    /// Bitwise NOT (unary, requires integer)
    pub fn op_not(&mut self) -> Result<(), VmError> {
        let a = self.memory.pop()?;
        let ai = self.require_int_value(a)?;
        self.push_num(Number::Int(!ai))
    }

    /// Compares strings and numerics
    pub fn compare_generic(&mut self, opcode: Opcode) -> Result<(), VmError> {
        let vb = self.memory.pop()?;
        let va = self.memory.pop()?;

        // Both refs and both strings
        if let (Value::Ref(a_addr), Value::Ref(b_addr)) = (&va, &vb) {
            let (ta, da) = self.heap_get_type_and_data(*a_addr)?;
            let (tb, db) = self.heap_get_type_and_data(*b_addr)?;
            if ta == Type::String && tb == Type::String {
                let sa = std::str::from_utf8(da).unwrap_or_default().to_string();
                let sb = std::str::from_utf8(db).unwrap_or_default().to_string();
                return self.push_comparison(sa, sb, opcode);
            }
        }

        // Numeric
        let na = va.as_num()?;
        let nb = vb.as_num()?;
        self.push_comparison(na.to_f32(), nb.to_f32(), opcode)
    }

    fn push_comparison<T: PartialOrd>(
        &mut self,
        a: T,
        b: T,
        opcode: Opcode,
    ) -> Result<(), VmError> {
        let res = match opcode {
            Opcode::EQ => (a == b) as i32,
            Opcode::NE => (a != b) as i32,
            Opcode::LT => (a < b) as i32,
            Opcode::GT => (a > b) as i32,
            Opcode::LE => (a <= b) as i32,
            Opcode::GE => (a >= b) as i32,
            _ => unreachable!(),
        };
        self.push_num(Number::Int(res))
    }

    /// Pop reference address (an address to a heap object) and push value or ref
    pub fn op_load(&mut self) -> Result<(), VmError> {
        let addr = self.pop_ref()?;
        let (ty, data) = self.heap_get_type_and_data(addr)?;
        match ty {
            Type::Integer => {
                let bytes = self.extract_4_bytes(data, addr)?;
                let val = i32::from_le_bytes(bytes);
                self.push_num(Number::Int(val))?;
            }
            Type::Float => {
                let bytes = self.extract_4_bytes(data, addr)?;
                let val = f32::from_le_bytes(bytes);
                self.push_num(Number::Float(val))?;
            }
            // For complex types push the reference to the heap object
            Type::String | Type::Reference(_) | Type::Array(_) | Type::FixedArray(_, _) => {
                self.push_ref(addr)?;
            }
            _ => return Err(VmError::type_mismatch("loadable", format!("{:?}", ty))),
        }
        Ok(())
    }

    /// Pop value, address, and write to target based on type
    pub fn op_store(&mut self) -> Result<(), VmError> {
        let val = self.memory.pop()?;
        let addr = self.pop_ref()?;

        // If value is numeric and target expects numeric, write payload
        if let Value::Imm(num) = val {
            let ty = self.heap_get_type(addr)?;
            if ty != Type::Integer || ty != Type::Float {
                return Err(VmError::type_mismatch(
                    "integer or float target",
                    format!("{:?}", ty),
                ));
            }

            // Write to heap
            let mut buf = ty.to_bytes();
            match num {
                Number::Int(i) => buf.extend(&i.to_le_bytes()),
                Number::Float(f) => buf.extend(&f.to_le_bytes()),
            }
            self.memory.write_bytes(addr, &buf)?;

            return Ok(());
        }

        // If val is a ref, copy the source object's RTTI+data into destination
        if let Value::Ref(src_addr) = val {
            let (rtti_src, data_src) = self.memory.load_from_heap(src_addr)?;
            // Write to heap
            let mut buf = Vec::with_capacity(rtti_src.len() + data_src.len());
            buf.extend_from_slice(rtti_src);
            buf.extend_from_slice(data_src);
            self.memory.write_bytes(addr, &buf)?;

            return Ok(());
        }

        Err(VmError::type_mismatch("storable", "value"))
    }

    pub fn op_syscall(&mut self, id: u8) -> Result<(), VmError> {
        // Convert operant into ID
        let id = SysCallId::from_u8(id).map_err(|e| VmError::type_mismatch("syscall id", e))?;

        // Pop args count
        let argc_val = self.memory.pop()?;
        let argc = match argc_val {
            Value::Imm(i) if i.to_i32() >= 0 => i.to_i32() as usize,
            Value::Imm(_) => {
                return Err(VmError::type_mismatch(
                    "non-negative integer",
                    format!("arg count {:?}", argc_val),
                ));
            }
            Value::Ref(_) => {
                return Err(VmError::type_mismatch(
                    "integer",
                    format!("arg count {:?}", argc_val),
                ));
            }
        };

        // Pop arguments
        let mut args = Vec::with_capacity(argc);
        for _ in 0..argc {
            args.push(self.memory.pop()?);
        }
        args.reverse();

        // Call syscalls
        match id {
            SysCallId::Print => self.op_print(&args),
            SysCallId::Println => self.op_println(&args),
            SysCallId::Input => self.op_input(&args),
        }?;

        Ok(())
    }

    fn op_print(&mut self, args: &[Value]) -> Result<(), VmError> {
        for val in args {
            let s = self.value_to_string(val)?;
            print!("{}", s);
            io::stdout().flush().unwrap();
        }
        Ok(())
    }

    fn op_println(&mut self, args: &[Value]) -> Result<(), VmError> {
        for val in args {
            let s = self.value_to_string(val)?;
            println!("{}", s);
        }
        Ok(())
    }

    fn op_input(&mut self, args: &[Value]) -> Result<(), VmError> {
        // Print prompt
        self.op_print(args)?;

        // Get line
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .map_err(|e| VmError::runtime(e.to_string()))?;

        // Save data to heap
        let (rtti, data) = self.build_data(input, Type::String)?;
        let addr = self.memory.save_to_heap(&rtti, &data, false)?;

        // Push reference to stack
        self.push_ref(addr)?;

        Ok(())
    }
}
