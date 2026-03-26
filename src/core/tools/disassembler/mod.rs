use crate::core::shared::bytecode::{Instruction, Opcode};
use crate::core::shared::executable::Executable;

/// Disassembles an Executable into human-readable format.
pub struct Disassembler;

impl Disassembler {
    pub fn disassemble(executable: &Executable) -> String {
        let mut output = String::new();

        output.push_str("BYTECODE DISASSEMBLY\n");
        output.push_str("====================\n\n");

        Self::write_header(&mut output, executable);
        Self::write_data_section(&mut output, executable);
        Self::write_code_section(&mut output, executable);

        output.push_str("\n====================\n");
        output.push_str("END OF DISASSEMBLY\n");

        output
    }

    fn write_header(output: &mut String, executable: &Executable) {
        output.push_str("FILE HEADER\n");
        output.push_str("-----------\n");
        output.push_str(&format!(
            "  Instructions: {}\n",
            executable.instruction_count()
        ));
        output.push_str(&format!(
            "  Data Size:    {} bytes\n\n",
            executable.data_size()
        ));
    }

    fn write_data_section(output: &mut String, executable: &Executable) {
        if executable.data_size() == 0 {
            return;
        }

        output.push_str("DATA SECTION\n");
        output.push_str("-----------\n");

        let data = executable.data();

        // Calculate max hex dump width for alignment
        let max_hex_width = data
            .chunks(16)
            .map(|chunk| {
                chunk
                    .iter()
                    .map(|b| format!("{:02x}", b))
                    .collect::<Vec<_>>()
                    .join(" ")
                    .len()
            })
            .max()
            .unwrap_or(0);

        output.push_str(&format!(
            "  Offset   Hex Dump{:width$}ASCII\n",
            "",
            width = max_hex_width + 2 - "Hex Dump".len()
        ));

        for (i, chunk) in data.chunks(16).enumerate() {
            let offset = i * 16;
            let hex_str = chunk
                .iter()
                .map(|b| format!("{:02x}", b))
                .collect::<Vec<_>>()
                .join(" ");

            let ascii_str = chunk
                .iter()
                .map(|&b| {
                    if (32..=126).contains(&b) {
                        b as char
                    } else {
                        '.'
                    }
                })
                .collect::<String>();

            output.push_str(&format!(
                "  0x{:04x}   {:<width$}  {}\n",
                offset,
                hex_str,
                ascii_str,
                width = max_hex_width
            ));
        }
        output.push('\n');
    }

    fn write_code_section(output: &mut String, executable: &Executable) {
        output.push_str("CODE SECTION\n");
        output.push_str("-----------\n");
        output.push_str("  Address  Instruction\n");
        output.push_str("  -------  ----------------------\n");

        for offset in 0..executable.instruction_count() {
            if let Some(instruction) = executable.get_instruction(offset) {
                let instruction_display = Self::format_instruction(&instruction);
                output.push_str(&format!("  0x{:06x}  {}\n", offset, instruction_display));
            }
        }
    }

    fn format_instruction(instruction: &Instruction) -> String {
        match instruction.opcode {
            // Stack manipulation
            Opcode::PUSH => format!("PUSH 0x{:x}", instruction.operand),
            Opcode::POP => "POP".to_string(),
            Opcode::DUP => "DUP".to_string(),
            Opcode::SWAP => "SWAP".to_string(),
            Opcode::ROT => "ROT".to_string(),

            // Arithmetic operations
            Opcode::ADD => "ADD".to_string(),
            Opcode::SUB => "SUB".to_string(),
            Opcode::MUL => "MUL".to_string(),
            Opcode::DIV => "DIV".to_string(),
            Opcode::REM => "REM".to_string(),
            Opcode::POW => "POW".to_string(),
            Opcode::NEG => "NEG".to_string(),

            // Logical operations
            Opcode::AND => "AND".to_string(),
            Opcode::OR => "OR".to_string(),
            Opcode::XOR => "XOR".to_string(),
            Opcode::NOT => "NOT".to_string(),

            // Bitwise shift operations
            Opcode::SLA => "SLA".to_string(),
            Opcode::SRA => "SRA".to_string(),

            // Comparison operations
            Opcode::EQ => "EQ".to_string(),
            Opcode::NE => "NE".to_string(),
            Opcode::LT => "LT".to_string(),
            Opcode::GT => "GT".to_string(),
            Opcode::LE => "LE".to_string(),
            Opcode::GE => "GE".to_string(),

            // Local variable access
            Opcode::LOAD_LOCAL => format!("LOAD.LOCAL {}", instruction.operand),
            Opcode::STORE_LOCAL => format!("STORE.LOCAL {}", instruction.operand),

            // Memory access
            Opcode::LOAD => "LOAD".to_string(),
            Opcode::STORE => "STORE".to_string(),

            // Control flow (operands are resolved addresses)
            Opcode::JMP => format!("JMP 0x{:06x}", instruction.operand),
            Opcode::JMPT => format!("JMPT 0x{:06x}", instruction.operand),
            Opcode::JMPF => format!("JMPF 0x{:06x}", instruction.operand),
            Opcode::CALL => format!("CALL 0x{:06x}", instruction.operand),
            Opcode::RET => "RET".to_string(),

            // Miscellaneous
            Opcode::NOP => "NOP".to_string(),
            Opcode::HALT => "HALT".to_string(),
        }
    }
}

pub trait DisassembleExt {
    fn disassemble(&self) -> String;
}

impl DisassembleExt for Executable {
    fn disassemble(&self) -> String {
        Disassembler::disassemble(self)
    }
}
