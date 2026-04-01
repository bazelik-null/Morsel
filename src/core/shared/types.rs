use std::fmt;
use std::fmt::Formatter;

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Integer,                      // 0x0
    Float,                        // 0x1
    Boolean,                      // 0x2
    String,                       // 0x3
    Array(Box<Type>),             // 0x4
    FixedArray(Box<Type>, usize), // 0x5
    Void,                         // 0x6
    Reference(Box<Type>),         // 0x7
}

impl Type {
    /// Serializes the Type into a 16-byte RTTI
    pub fn to_bytes(&self) -> [u8; 16] {
        let mut bytes = [0u8; 16];
        self.serialize_into(&mut bytes, 0);
        bytes
    }

    /// Recursively serialize into a buffer at a given offset
    /// Returns the number of bytes written
    fn serialize_into(&self, bytes: &mut [u8], mut offset: usize) -> usize {
        if offset >= bytes.len() {
            return 0;
        }

        let start_offset = offset;

        match self {
            Type::Integer => {
                bytes[offset] = 0x0;
                offset += 1;
            }
            Type::Float => {
                bytes[offset] = 0x1;
                offset += 1;
            }
            Type::Boolean => {
                bytes[offset] = 0x2;
                offset += 1;
            }
            Type::String => {
                bytes[offset] = 0x3;
                offset += 1;
            }
            Type::Array(inner) => {
                bytes[offset] = 0x4;
                offset += 1;
                offset += inner.serialize_into(bytes, offset);
            }
            Type::FixedArray(inner, size) => {
                bytes[offset] = 0x5;
                offset += 1;
                offset += inner.serialize_into(bytes, offset);
                // Serialize the size as 8 bytes (u64)
                if offset + 8 <= bytes.len() {
                    bytes[offset..offset + 8].copy_from_slice(&size.to_le_bytes());
                    offset += 8;
                }
            }
            Type::Void => {
                bytes[offset] = 0x6;
                offset += 1;
            }
            Type::Reference(inner) => {
                bytes[offset] = 0x7;
                offset += 1;
                offset += inner.serialize_into(bytes, offset);
            }
        }

        offset - start_offset
    }

    /// Deserializes bytes back into a Type
    /// Returns (Type, bytes_consumed)
    pub fn from_bytes(bytes: &[u8]) -> Result<(Self, usize), String> {
        if bytes.is_empty() {
            return Err("Empty byte slice".to_string());
        }

        match bytes[0] {
            0x0 => Ok((Type::Integer, 1)),
            0x1 => Ok((Type::Float, 1)),
            0x2 => Ok((Type::Boolean, 1)),
            0x3 => Ok((Type::String, 1)),
            0x4 => {
                let (inner, consumed) = Type::from_bytes(&bytes[1..])?;
                Ok((Type::Array(Box::new(inner)), 1 + consumed))
            }
            0x5 => {
                let (inner, consumed) = Type::from_bytes(&bytes[1..])?;
                let size_start = 1 + consumed;
                if bytes.len() < size_start + 8 {
                    return Err("Not enough bytes for FixedArray size".to_string());
                }
                let size_bytes: [u8; 8] = bytes[size_start..size_start + 8]
                    .try_into()
                    .map_err(|_| "Failed to parse size".to_string())?;
                let size = usize::from_le_bytes(size_bytes);
                Ok((Type::FixedArray(Box::new(inner), size), size_start + 8))
            }
            0x6 => Ok((Type::Void, 1)),
            0x7 => {
                let (inner, consumed) = Type::from_bytes(&bytes[1..])?;
                Ok((Type::Reference(Box::new(inner)), 1 + consumed))
            }
            _ => Err(format!("Unknown type tag: {}", bytes[0])),
        }
    }

    /// Get byte offsets where pointers are stored in serialized data
    pub fn pointer_offsets(&self) -> Vec<usize> {
        match self {
            Type::Integer | Type::Float | Type::Boolean | Type::Void => {
                vec![] // No pointers
            }
            Type::String => {
                vec![] // String is just data
            }
            Type::Reference(_) => {
                vec![0] // Reference itself is the pointer
            }
            Type::Array(_) => {
                // Array layout: [length: u32][capacity: u32][ptr: u64]
                vec![8] // Pointer at offset 8
            }
            Type::FixedArray(element_type, size) => {
                // Fixed array: elements laid out sequentially
                let element_size = element_type.size_in_bytes();
                let mut offsets = Vec::new();
                if element_type.contains_references() {
                    for i in 0..*size {
                        offsets.extend(
                            element_type
                                .pointer_offsets()
                                .iter()
                                .map(|o| i * element_size + o),
                        );
                    }
                }
                offsets
            }
        }
    }

    pub fn contains_references(&self) -> bool {
        match self {
            Type::Reference(_) => true,
            Type::Array(inner) | Type::FixedArray(inner, _) => inner.contains_references(),
            _ => false,
        }
    }

    pub fn size_in_bytes(&self) -> usize {
        match self {
            Type::Integer => 4,
            Type::Float => 4,
            Type::Boolean => 1,
            Type::String => 0, // Variable length, stored as data not in type
            Type::Reference(_) => 8,
            Type::Array(_) => 16, // [len:4][cap:4][ptr:8]
            Type::FixedArray(element_type, size) => element_type.size_in_bytes() * size,
            Type::Void => 0,
        }
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Type::Integer => write!(f, "int"),
            Type::Float => write!(f, "float"),
            Type::Boolean => write!(f, "bool"),
            Type::String => write!(f, "string"),
            Type::Array(inner) => write!(f, "[{}]", inner),
            Type::FixedArray(inner, size) => write!(f, "[{}: {}]", inner, size),
            Type::Void => write!(f, "void"),
            Type::Reference(inner) => write!(f, "ref {}", inner),
        }
    }
}
