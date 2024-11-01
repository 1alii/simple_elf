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
            },
        ))
    }
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct Elf64_Rel {
    pub r_offset: u64, /* Address of reference */
    pub r_info: u64,   /* Symbol index and type of relocation */
}

impl Into<Vec<u8>> for Elf64_Rel {
    fn into(self) -> Vec<u8> {
        let mut out: Vec<u8> = Vec::new();
        out.extend(self.r_offset.to_le_bytes());
        out.extend(self.r_info.to_le_bytes());
        out
    }
}

impl Into<Vec<u8>> for &Elf64_Rel {
    fn into(self) -> Vec<u8> {
        let mut out: Vec<u8> = Vec::new();
        out.extend(self.r_offset.to_le_bytes());
        out.extend(self.r_info.to_le_bytes());
        out
    }
}

impl Elf64_Rel {
    pub fn parse(raw: &[u8]) -> IResult<&[u8], Self> {
        let (remaining, r_offset) = number::complete::le_u64(raw)?;
        let (remaining, r_info) = number::complete::le_u64(remaining)?;
        Ok((remaining, Self { r_info, r_offset }))
    }
}

#[derive(Clone, Copy, Debug, Serialize)]
#[repr(u32)]
pub enum RelocationType {
    R_X86_64_NONE = 0,             // none none
    R_X86_64_64 = 1,               // word64 S + A
    R_X86_64_PC32 = 2,             // word32 S + A - P
    R_X86_64_GOT32 = 3,            // word32 G + A
    R_X86_64_PLT32 = 4,            // word32 L + A - P
    R_X86_64_COPY = 5,             // none none
    R_X86_64_GLOB_DAT = 6,         // wordclass S
    R_X86_64_JUMP_SLOT = 7,        // wordclass S
    R_X86_64_RELATIVE = 8,         // wordclass B + A
    R_X86_64_GOTPCREL = 9,         // word32 G + GOT + A - P
    R_X86_64_32 = 10,              // word32 S + A
    R_X86_64_32S = 11,             // word32 S + A
    R_X86_64_16 = 12,              // word16 S + A
    R_X86_64_PC16 = 13,            // word16 S + A - P
    R_X86_64_8 = 14,               // word8 S + A
    R_X86_64_PC8 = 15,             // word8 S + A - P
    R_X86_64_DTPMOD64 = 16,        // word64
    R_X86_64_DTPOFF64 = 17,        // word64
    R_X86_64_TPOFF64 = 18,         // word64
    R_X86_64_TLSGD = 19,           // word32
    R_X86_64_TLSLD = 20,           // word32
    R_X86_64_DTPOFF32 = 21,        // word32
    R_X86_64_GOTTPOFF = 22,        // word32
    R_X86_64_TPOFF32 = 23,         // word32
    R_X86_64_PC64 = 24,            // word64 S + A - P
    R_X86_64_GOTOFF64 = 25,        // word64 S + A - GOT
    R_X86_64_GOTPC32 = 26,         // word32 GOT + A - P
    R_X86_64_SIZE32 = 32,          // word32 Z + A
    R_X86_64_SIZE64 = 33,          // word64 Z + A
    R_X86_64_GOTPC32_TLSDESC = 34, // word32
    R_X86_64_TLSDESC_CALL = 35,    // none
    R_X86_64_TLSDESC = 36,         // word64×2
    R_X86_64_IRELATIVE = 37,       // wordclass indirect (B + A)
    R_X86_64_RELATIVE64 = 38,      // word64 B + A
    // Deprecated 39,
    // Deprecated 40,
    R_X86_64_GOTPCRELX = 41,              // word32 G + GOT + A - P
    R_X86_64_REX_GOTPCRELX = 42,          // word32 G + GOT + A - P
    R_X86_64_CODE_4_GOTPCRELX = 43,       // word32 G + GOT + A - P
    R_X86_64_CODE_4_GOTTPOFF = 44,        // word32
    R_X86_64_CODE_4_GOTPC32_TLSDESC = 45, // word32
    R_X86_64_CODE_5_GOTPCRELX = 46,       // word32 G + GOT + A - P
    R_X86_64_CODE_5_GOTTPOFF = 47,        // word32
    R_X86_64_CODE_5_GOTPC32_TLSDESC = 48, // word32
    R_X86_64_CODE_6_GOTPCRELX = 49,       // word32 G + GOT + A - P
    R_X86_64_CODE_6_GOTTPOFF = 50,        // word32
    R_X86_64_CODE_6_GOTPC32_TLSDESC = 51,
    UNSPECIFIED(u32),
}

impl From<u32> for RelocationType {
    fn from(value: u32) -> Self {
        match value {
            0 => Self::R_X86_64_NONE,
            1 => Self::R_X86_64_64,
            2 => Self::R_X86_64_PC32,
            3 => Self::R_X86_64_GOT32,
            4 => Self::R_X86_64_PLT32,
            5 => Self::R_X86_64_COPY,
            6 => Self::R_X86_64_GLOB_DAT,
            7 => Self::R_X86_64_JUMP_SLOT,
            8 => Self::R_X86_64_RELATIVE,
            9 => Self::R_X86_64_GOTPCREL,
            10 => Self::R_X86_64_32,
            11 => Self::R_X86_64_32S,
            12 => Self::R_X86_64_16,
            13 => Self::R_X86_64_PC16,
            14 => Self::R_X86_64_8,
            15 => Self::R_X86_64_PC8,
            16 => Self::R_X86_64_DTPMOD64,
            17 => Self::R_X86_64_DTPOFF64,
            18 => Self::R_X86_64_TPOFF64,
            19 => Self::R_X86_64_TLSGD,
            20 => Self::R_X86_64_TLSLD,
            21 => Self::R_X86_64_DTPOFF32,
            22 => Self::R_X86_64_GOTTPOFF,
            23 => Self::R_X86_64_TPOFF32,
            24 => Self::R_X86_64_PC64,
            25 => Self::R_X86_64_GOTOFF64,
            26 => Self::R_X86_64_GOTPC32,
            32 => Self::R_X86_64_SIZE32,
            33 => Self::R_X86_64_SIZE64,
            34 => Self::R_X86_64_GOTPC32_TLSDESC,
            35 => Self::R_X86_64_TLSDESC_CALL,
            36 => Self::R_X86_64_TLSDESC,
            37 => Self::R_X86_64_IRELATIVE,
            38 => Self::R_X86_64_RELATIVE64,
            41 => Self::R_X86_64_GOTPCRELX,
            42 => Self::R_X86_64_REX_GOTPCRELX,
            43 => Self::R_X86_64_CODE_4_GOTPCRELX,
            44 => Self::R_X86_64_CODE_4_GOTTPOFF,
            45 => Self::R_X86_64_CODE_4_GOTPC32_TLSDESC,
            46 => Self::R_X86_64_CODE_5_GOTPCRELX,
            47 => Self::R_X86_64_CODE_5_GOTTPOFF,
            48 => Self::R_X86_64_CODE_5_GOTPC32_TLSDESC,
            49 => Self::R_X86_64_CODE_6_GOTPCRELX,
            50 => Self::R_X86_64_CODE_6_GOTTPOFF,
            51 => Self::R_X86_64_CODE_6_GOTPC32_TLSDESC,
            _ => Self::UNSPECIFIED(value),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct Elf64_Rela {
    pub r_offset: u64, /* Address of reference */
    pub r_info: u64,   /* Symbol index and type of relocation */
    pub r_addend: u64, /* Constant part of expression */
    pub relocation_type: RelocationType,
    pub symbol_index: u32,
}

impl Into<Vec<u8>> for Elf64_Rela {
    fn into(self) -> Vec<u8> {
        let mut out: Vec<u8> = Vec::new();
        out.extend(self.r_offset.to_le_bytes());
        out.extend(self.r_info.to_le_bytes());
        out.extend(self.r_addend.to_le_bytes());
        out
    }
}

impl Into<Vec<u8>> for &Elf64_Rela {
    fn into(self) -> Vec<u8> {
        let mut out: Vec<u8> = Vec::new();
        out.extend(self.r_offset.to_le_bytes());
        out.extend(self.r_info.to_le_bytes());
        out.extend(self.r_addend.to_le_bytes());
        out
    }
}

impl Elf64_Rela {
    pub fn parse(raw: &[u8]) -> IResult<&[u8], Self> {
        let (remaining, r_offset) = number::complete::le_u64(raw)?;
        let (remaining, r_info) = number::complete::le_u64(remaining)?;
        let (remaining, r_addend) = number::complete::le_u64(remaining)?;

        Ok((
            remaining,
            Self {
                r_info,
                r_offset,
                r_addend,
                relocation_type: ((r_info & 0xffffffff) as u32).into(),
                symbol_index: (r_info >> 32) as u32,
            },
        ))
    }
}
