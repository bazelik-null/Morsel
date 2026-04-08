use crate::core::shared::bytecode::Opcode;
use crate::core::shared::executable::Executable;
use crate::core::shared::types::Type;
use colored::Colorize;
use std::collections::HashSet;

/// Disassembles an Executable into human-readable format.
pub struct Disassembler;

impl Disassembler {
    pub fn disassemble(executable: &Executable) -> String {
        let mut output = String::new();

        output.push_str(&"\n BYTECODE DISASSEMBLY\n".bold().cyan().to_string());
        output.push_str(&"═".repeat(75).cyan().to_string());
        output.push('\n');

        Self::write_header(&mut output, executable);
        Self::write_data_section(&mut output, executable);
        Self::write_code_section(&mut output, executable);

        output.push('\n');
        output.push_str(&"═".repeat(75).cyan().to_string());
        output.push('\n');
        output.push_str(&" END OF DISASSEMBLY\n".bold().cyan().to_string());

        output
    }

    /// Collect all function entry points from CALL instructions
    fn collect_function_entries(executable: &Executable) -> HashSet<usize> {
        let mut entries = HashSet::new();

        // Entry point is always a function
        entries.insert(executable.header.entry_point as usize);

        // Scan for CALL instructions to find other function entry points
        for offset in 0..executable.instructions.len() {
            if let Some(instruction) = executable.instructions.get(offset)
                && (instruction.opcode == Opcode::CALL)
            {
                entries.insert(instruction.operand as usize);
            }
        }

        entries
    }

    /// Generate function labels for display
    fn generate_function_label(entry_point: usize, is_main: bool) -> String {
        if is_main {
            ".main".to_string()
        } else {
            format!(".func_{:x}", entry_point)
        }
    }

    fn write_header(output: &mut String, executable: &Executable) {
        output.push_str(&" FILE HEADER\n".bold().yellow().to_string());
        output.push_str(&"─".repeat(75).yellow().to_string());
        output.push('\n');
        output.push_str(&format!(
            "  {} {}\n",
            "Instructions:".bright_white(),
            executable.instructions.len().to_string().green()
        ));
        output.push_str(&format!(
            "  {} {} bytes\n",
            "Data Size:".bright_white(),
            executable.data.len().to_string().green()
        ));
        output.push_str(&format!(
            "  {} 0x{:06x}\n",
            "Entry Point:".bright_white(),
            executable.header.entry_point
        ));
        output.push('\n');
    }

    fn write_data_section(output: &mut String, executable: &Executable) {
        if executable.data.is_empty() {
            return;
        }

        output.push_str(&" DATA SECTION\n".bold().yellow().to_string());
        output.push_str(&"─".repeat(75).yellow().to_string());
        output.push('\n');

        // Column header
        output.push_str(&format!(
            "  {:<10} {:<8} {}\n\n",
            "Address".bright_white().bold(),
            "Type".bright_white().bold(),
            "Data".bright_white().bold()
        ));

        let data = &executable.data;
        let mut off = 0usize;

        while off < data.len() {
            // Need header (16 bytes)
            if off + 16 > data.len() {
                let rem = &data[off..];
                let hex = rem
                    .iter()
                    .map(|b| format!("{:06x}", b))
                    .collect::<Vec<_>>()
                    .join(" ");
                output.push_str(&format!(
                    "  {}  {:<20} {}\n",
                    format!("0x{:06x}", off).cyan(),
                    "<truncated header>".bright_red(),
                    hex.bright_black()
                ));
                break;
            }

            let total_size = u32::from_le_bytes(data[off..off + 4].try_into().unwrap()) as usize;
            let rtti_bytes = &data[off + 4..off + 12];
            let data_len =
                u32::from_le_bytes(data[off + 12..off + 16].try_into().unwrap()) as usize;
            let payload_start = off + 16;
            let payload_end = payload_start.saturating_add(data_len);

            // Validate
            if total_size < 16 || payload_end > data.len() || total_size != 16 + data_len {
                let snippet = &data[off..std::cmp::min(data.len(), off + 32)];
                let hex = snippet
                    .iter()
                    .map(|b| format!("{:02x}", b))
                    .collect::<Vec<_>>()
                    .join(" ");
                output.push_str(&format!(
                    "  {}  {:<20} {}\n",
                    format!("0x{:06x}", off).cyan(),
                    "<malformed>".bright_red(),
                    hex.bright_black()
                ));
                off += 1;
                continue;
            }

            let payload = &data[payload_start..payload_end];

            // RTTI
            let type_label = match Type::from_bytes(rtti_bytes) {
                Ok((t, _)) => format!("{}", t).green().to_string(),
                Err(_) => format!(
                    "RTTI(0x{})",
                    rtti_bytes
                        .iter()
                        .map(|b| format!("{:02x}", b))
                        .collect::<String>()
                )
                .bright_red()
                .to_string(),
            };

            let data_label = match Type::from_bytes(rtti_bytes) {
                Ok((t, _)) => match t {
                    Type::Integer => {
                        if payload.len() >= 4 {
                            let v = i32::from_le_bytes(payload[..4].try_into().unwrap());
                            format!("int({})", v)
                        } else {
                            format!("int(<{} bytes>)", payload.len())
                        }
                    }
                    Type::Float => {
                        if payload.len() >= 4 {
                            let v = f32::from_le_bytes(payload[..4].try_into().unwrap());
                            format!("float({})", v)
                        } else {
                            format!("float(<{} bytes>)", payload.len())
                        }
                    }
                    Type::Boolean => {
                        let b = payload.first().map(|x| *x != 0).unwrap_or(false);
                        format!("bool({})", b)
                    }
                    Type::String => match std::str::from_utf8(payload) {
                        Ok(s) => format!("\"{}\"", s),
                        Err(_) => format!("string(<invalid utf8, {} bytes>)", payload.len()),
                    },
                    Type::Array(_) => {
                        if payload.len() >= 4 {
                            let cnt = u32::from_le_bytes(payload[..4].try_into().unwrap());
                            format!("array(len={})", cnt)
                        } else {
                            "array(<truncated>)".to_string()
                        }
                    }
                    Type::FixedArray(elem, size) => {
                        let mut elems = Vec::new();
                        let mut cur = 0;
                        for _ in 0..size {
                            if cur >= payload.len() {
                                elems.push("<trunc>".to_string());
                                break;
                            }
                            match *elem {
                                Type::Integer => {
                                    if cur + 4 <= payload.len() {
                                        elems.push(format!(
                                            "{}",
                                            i32::from_le_bytes(
                                                payload[cur..cur + 4].try_into().unwrap()
                                            )
                                        ));
                                        cur += 4;
                                    } else {
                                        elems.push("<trunc>".to_string());
                                        break;
                                    }
                                }
                                Type::Float => {
                                    if cur + 4 <= payload.len() {
                                        elems.push(format!(
                                            "{}",
                                            f32::from_le_bytes(
                                                payload[cur..cur + 4].try_into().unwrap()
                                            )
                                        ));
                                        cur += 4;
                                    } else {
                                        elems.push("<trunc>".to_string());
                                        break;
                                    }
                                }
                                Type::String => {
                                    elems.push("<string>".to_string());
                                    break;
                                }
                                _ => {
                                    elems.push("<val>".to_string());
                                    break;
                                }
                            }
                        }
                        format!("[{}]", elems.join(", "))
                    }
                    Type::Reference(_) | Type::MutableReference(_) => {
                        if payload.len() >= 4 {
                            format!(
                                "ref(0x{:08x})",
                                u32::from_le_bytes(payload[..4].try_into().unwrap())
                            )
                        } else {
                            "ref(<trunc>)".to_string()
                        }
                    }
                    Type::Void => "void".to_string(),
                },
                Err(_) => {
                    // fallback hex
                    let hex = payload
                        .iter()
                        .map(|b| format!("{:02x}", b))
                        .collect::<Vec<_>>()
                        .join(" ");
                    format!("0x{}", hex)
                }
            };

            // Trim long data_label
            let data_trim = if data_label.len() > 60 {
                format!("{}...", &data_label[..57])
            } else {
                data_label
            };

            output.push_str(&format!(
                "  {}   {:<17} {}\n",
                format!("0x{:06x}", off).cyan(),
                type_label,
                data_trim.bright_black()
            ));

            off += total_size;
        }

        output.push('\n');
    }

    fn write_code_section(output: &mut String, executable: &Executable) {
        output.push_str(&" CODE SECTION\n".bold().yellow().to_string());
        output.push_str(&"─".repeat(75).yellow().to_string());
        output.push('\n');
        output.push_str(&format!(
            "  {:<9} {}\n",
            "Address".bright_white().bold(),
            "Instruction".bright_white().bold()
        ));

        // Build function entry point map
        let function_entries = Self::collect_function_entries(executable);
        let mut sorted_entries: Vec<usize> = function_entries.into_iter().collect();
        sorted_entries.sort();

        let entry_point = executable.header.entry_point as usize;
        let mut function_labels: std::collections::HashMap<usize, String> =
            std::collections::HashMap::new();

        for entry in sorted_entries.iter() {
            let is_main = *entry == entry_point;
            function_labels.insert(*entry, Self::generate_function_label(*entry, is_main));
        }

        for offset in 0..executable.instructions.len() {
            // Print function label if this address is a function entry point
            if let Some(label) = function_labels.get(&offset) {
                output.push_str(&format!("\n  {}\n", label.bold().green()));
            }

            if let Some(instruction) = executable.instructions.get(offset) {
                output.push_str(&format!(
                    "  {}  {}\n",
                    format!("0x{:06x}", offset).cyan(),
                    instruction
                ));
            }
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
