#![allow(non_camel_case_types)]

use nom::number;
use nom::IResult;
use serde::Serialize;

#[derive(Debug, Clone, Copy, Serialize)]
#[repr(u16)]
pub enum EMachine {
    ET_NONE = 0,         // No machine
    EM_M32 = 1,          // AT&T WE 32100
    EM_SPARC = 2,        // SPARC
    EM_386 = 3,          // Intel Architecture
    EM_68K = 4,          // Motorola 68000
    EM_88K = 5,          // Motorola 88000
    EM_860 = 7,          // Intel 80860
    EM_MIPS = 8,         // MIPS RS3000 Big-Endian
    EM_MIPS_RS4_BE = 10, // MIPS RS4000 Big-Endian
    EM_X86_64 = 62,
    UNSPECIFIED(u16),
}

impl EMachine {
    pub fn parse(raw: &[u8]) -> IResult<&[u8], Self> {
        let (r, c) = number::complete::le_u16(raw)?;
        Ok((r, Self::from(c)))
    }
}

impl From<u16> for EMachine {
    fn from(value: u16) -> Self {
        match value {
            0 => Self::ET_NONE,
            1 => Self::EM_M32,
            2 => Self::EM_SPARC,
            3 => Self::EM_386,
            4 => Self::EM_68K,
            5 => Self::EM_88K,
            7 => Self::EM_860,
            8 => Self::EM_MIPS,
            10 => Self::EM_MIPS_RS4_BE,
            62 => Self::EM_X86_64,
            _ => Self::UNSPECIFIED(value),
        }
    }
}

impl Into<u16> for EMachine {
    fn into(self) -> u16 {
        match self {
            Self::EM_386 => 3,
            Self::EM_68K => 4,
            Self::EM_860 => 7,
            Self::EM_88K => 5,
            Self::EM_M32 => 1,
            Self::EM_MIPS => 8,
            Self::EM_MIPS_RS4_BE => 10,
            Self::EM_SPARC => 2,
            Self::ET_NONE => 0,
            Self::EM_X86_64 => 62,
            Self::UNSPECIFIED(v) => v,
        }
    }
}

impl Into<Vec<u8>> for EMachine {
    fn into(self) -> Vec<u8> {
        let tmp: u16 = self.into();
        tmp.to_le_bytes().into()
    }
}

impl Into<Vec<u8>> for &EMachine {
    fn into(self) -> Vec<u8> {
        let tmp: u16 = (*self).into();
        tmp.to_le_bytes().into()
    }
}
