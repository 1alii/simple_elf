#![allow(non_camel_case_types)]

use nom::number;
use nom::IResult;
use serde::Serialize;

#[derive(Debug, Clone, Copy, Serialize)]
pub enum SymbolBinding {
    STB_LOCAL,  // Not visible outside the object file
    STB_GLOBAL, // Global symbol, visible to all object files
    STB_WEAK,   // Global scope, but with lower precedence than global symbols
    UNSPECIFIED(u8),
}

impl Into<u8> for SymbolBinding {
    fn into(self) -> u8 {
        match self {
            Self::STB_LOCAL => 0,
            Self::STB_GLOBAL => 1,
            Self::STB_WEAK => 2,
            Self::UNSPECIFIED(v) => v,
        }
    }
}

impl From<u8> for SymbolBinding {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::STB_LOCAL,
            1 => Self::STB_GLOBAL,
            2 => Self::STB_WEAK,
            _ => Self::UNSPECIFIED(value),
        }
    }
}
#[derive(Debug, Clone, Copy, Serialize)]
pub enum SymbolType {
    STT_NOTYPE,  // No type specified (e.g., an absolute symbol)
    STT_OBJECT,  // Data object
    STT_FUNC,    // Function entry point
    STT_SECTION, // Symbol is associated with a section
    STT_FILE,    // Source file associated with the object file
    UNSPECIFIED(u8),
}

impl Into<u8> for SymbolType {
    fn into(self) -> u8 {
        match self {
            Self::STT_NOTYPE => 0,
            Self::STT_OBJECT => 1,
            Self::STT_FUNC => 2,
            Self::STT_SECTION => 3,
            Self::STT_FILE => 4,
            Self::UNSPECIFIED(v) => v,
        }
    }
}
impl From<u8> for SymbolType {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::STT_NOTYPE,
            1 => Self::STT_OBJECT,
            2 => Self::STT_FUNC,
            3 => Self::STT_SECTION,
            4 => Self::STT_FILE,
            _ => Self::UNSPECIFIED(value),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Elf64_Sym {
    pub st_name: u32,  /* Symbol name  if 0 it has no name */
    pub st_info: u8,   /* Type and Binding attributes */
    pub st_other: u8,  /* Reserved */
    pub st_shndx: u16, /* Section table index */
    pub st_value: u64, /* Symbol value */
    pub st_size: u64,  /* Size of object (e.g., common) */
    pub symbol_type: SymbolType,
    pub symbol_binding: SymbolBinding,
    pub name: String,
}

impl Into<Vec<u8>> for Elf64_Sym {
    fn into(self) -> Vec<u8> {
        let mut out: Vec<u8> = Vec::new();
        out.extend(self.st_name.to_le_bytes());
        out.push(self.st_info);
        out.push(self.st_other);
        out.extend(self.st_shndx.to_le_bytes());
        out.extend(self.st_value.to_le_bytes());
        out.extend(self.st_size.to_le_bytes());
        out
    }
}

impl Into<Vec<u8>> for &Elf64_Sym {
    fn into(self) -> Vec<u8> {
        let mut out: Vec<u8> = Vec::new();
        out.extend(self.st_name.to_le_bytes());
        out.push(self.st_info);
        out.push(self.st_other);
        out.extend(self.st_shndx.to_le_bytes());
        out.extend(self.st_value.to_le_bytes());
        out.extend(self.st_size.to_le_bytes());
        out
    }
}

impl Elf64_Sym {
    pub fn parse(raw: &[u8]) -> IResult<&[u8], Self> {
        let (remaining, st_name) = number::complete::le_u32(raw)?;
        let (remaining, st_info) = number::complete::le_u8(remaining)?;
        let (remaining, st_other) = number::complete::le_u8(remaining)?;
        let (remaining, st_shndx) = number::complete::le_u16(remaining)?;
        let (remaining, st_value) = number::complete::le_u64(remaining)?;
        let (remaining, st_size) = number::complete::le_u64(remaining)?;
        Ok((
            remaining,
            Self {
                st_info,
                st_name,
                st_size,
                st_other,
                st_shndx,
                st_value,
                symbol_type: SymbolType::from(st_info << 4),
                symbol_binding: SymbolBinding::from(st_info >> 4),
                name: String::new(),
            },
        ))
    }
}
