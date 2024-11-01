#![allow(non_camel_case_types)]

use nom::number;
use nom::IResult;
use serde::Serialize;

#[derive(Debug, Clone, Copy, Serialize)]
#[repr(u16)]
pub enum EType {
    /*
    0xFE00 to 0xFEFF : Environment-specific
    0xff00 to 0xffff : Processor-specific
     */
    ET_NONE = 0, // fmwf
    ET_REL = 1,  // Relocatable file
    ET_EXEC = 2, // Executable file
    ET_DYN = 3,  // Shared object file
    ET_CORE = 4, // Core file
    UNSPECIFIED(u16),
}

impl EType {
    pub fn parse(raw: &[u8]) -> IResult<&[u8], Self> {
        let (r, c) = number::complete::le_u16(raw)?;
        Ok((r, Self::from(c)))
    }
}

impl Into<Vec<u8>> for EType {
    fn into(self) -> Vec<u8> {
        let tmp: u16 = self.into();
        tmp.to_le_bytes().into()
    }
}

impl Into<Vec<u8>> for &EType {
    fn into(self) -> Vec<u8> {
        let tmp: u16 = (*self).into();
        tmp.to_le_bytes().into()
    }
}

impl From<u16> for EType {
    fn from(value: u16) -> Self {
        match value {
            0 => Self::ET_NONE,
            1 => Self::ET_REL,
            2 => Self::ET_EXEC,
            3 => Self::ET_DYN,
            4 => Self::ET_CORE,
            _ => Self::UNSPECIFIED(value),
        }
    }
}

impl Into<u16> for EType {
    fn into(self) -> u16 {
        match self {
            Self::ET_NONE => 0,
            Self::ET_REL => 1,
            Self::ET_EXEC => 2,
            Self::ET_DYN => 3,
            Self::ET_CORE => 4,
            Self::UNSPECIFIED(v) => v,
        }
    }
}
