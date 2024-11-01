#![allow(non_camel_case_types)]

use nom::number::complete::le_u32;
use nom::IResult;
use serde::Serialize;

#[derive(Debug, Clone, Copy, Serialize)]
#[repr(u32)]
pub enum PType {
    /**
     * 0x60000000 to 0x6FFFFFFF Environment-specific use
     * 0x70000000 to 0x7FFFFFFF Processor-specific use
     */
    PT_NULL = 0, // Unused entry
    PT_LOAD = 1,    // Loadable segment
    PT_DYNAMIC = 2, // Dynamic linking tables
    PT_INTERP = 3,  // Program interpreter path name
    PT_NOTE = 4,    // Note sections
    PT_SHLIB = 5,   // Reserved
    PT_PHDR = 6,    // Program header table
    PT_TLS = 7,
    PT_NUM = 8,
    PT_GNU_EH_FRAME = 0x6474e550,
    PT_GNU_STACK = 0x6474e551,
    PT_GNU_RELRO = 0x6474e552,
    PT_GNU_PROPERTY = 0x6474e553,
    UNSPECIFIED(u32),
}

impl Into<Vec<u8>> for PType {
    fn into(self) -> Vec<u8> {
        let tmp: u32 = self.into();
        tmp.to_le_bytes().into()
    }
}

impl Into<Vec<u8>> for &PType {
    fn into(self) -> Vec<u8> {
        let tmp: u32 = (*self).into();
        tmp.to_le_bytes().into()
    }
}

impl PType {
    pub fn parse(raw: &[u8]) -> IResult<&[u8], Self> {
        let (r, c) = le_u32(raw)?;
        Ok((r, Self::from(c)))
    }
}

impl From<u32> for PType {
    fn from(value: u32) -> Self {
        match value {
            0 => Self::PT_NULL,
            1 => Self::PT_LOAD,
            2 => Self::PT_DYNAMIC,
            3 => Self::PT_INTERP,
            4 => Self::PT_NOTE,
            5 => Self::PT_SHLIB,
            6 => Self::PT_PHDR,
            7 => Self::PT_TLS,
            8 => Self::PT_NUM,
            0x6474e550 => Self::PT_GNU_EH_FRAME,
            0x6474e551 => Self::PT_GNU_STACK,
            0x6474e552 => Self::PT_GNU_RELRO,
            0x6474e553 => Self::PT_GNU_PROPERTY,
            _ => Self::UNSPECIFIED(value),
        }
    }
}

impl Into<u32> for PType {
    fn into(self) -> u32 {
        match self {
            Self::PT_DYNAMIC => 2,
            Self::PT_INTERP => 3,
            Self::PT_LOAD => 1,
            Self::PT_NOTE => 4,
            Self::PT_NULL => 0,
            Self::PT_PHDR => 6,
            Self::PT_SHLIB => 5,
            Self::PT_TLS => 7,
            Self::PT_NUM => 8,
            Self::PT_GNU_EH_FRAME => 0x6474e550,
            Self::PT_GNU_STACK => 0x6474e551,
            Self::PT_GNU_RELRO => 0x6474e552,
            Self::PT_GNU_PROPERTY => 0x6474e553,
            Self::UNSPECIFIED(v) => v,
        }
    }
}
