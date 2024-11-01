#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use nom::bytes::complete::{tag, take};
use nom::number;
use nom::sequence;
use nom::IResult;
use serde::Serialize;

#[derive(Debug, Clone, Copy, Serialize)]
#[repr(u8)]
enum EI_CLASS {
    ELFCLASSNONE = 0, // Invalid class
    ELFCLASS32 = 1,   // 32-bit objects
    ELFCLASS64 = 2,   // 64-bit objects
    UNSPECIFIED(u8),
}

impl EI_CLASS {
    pub fn parse(raw: &[u8]) -> IResult<&[u8], Self> {
        let (r, c) = number::complete::le_u8(raw)?;
        Ok((r, Self::from(c)))
    }
}

impl From<u8> for EI_CLASS {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::ELFCLASSNONE,
            1 => Self::ELFCLASS32,
            2 => Self::ELFCLASS64,
            _ => Self::UNSPECIFIED(value),
        }
    }
}

impl Into<u8> for EI_CLASS {
    fn into(self) -> u8 {
        match self {
            Self::ELFCLASS32 => 1,
            Self::ELFCLASS64 => 2,
            Self::ELFCLASSNONE => 0,
            Self::UNSPECIFIED(v) => v,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize)]
#[repr(u8)]
enum EI_DATA {
    ELFDATANONE = 0, // Invalid data encoding
    ELFDATA2LSB = 1, // See below
    ELFDATA2MSB = 2, // See below
    UNSPECIFIED(u8),
}

impl EI_DATA {
    pub fn parse(raw: &[u8]) -> IResult<&[u8], Self> {
        let (r, c) = number::complete::le_u8(raw)?;
        Ok((r, Self::from(c)))
    }
}

impl From<u8> for EI_DATA {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::ELFDATANONE,
            1 => Self::ELFDATA2LSB,
            2 => Self::ELFDATA2MSB,
            _ => Self::UNSPECIFIED(value),
        }
    }
}

impl Into<u8> for EI_DATA {
    fn into(self) -> u8 {
        match self {
            Self::ELFDATA2LSB => 1,
            Self::ELFDATA2MSB => 2,
            Self::ELFDATANONE => 0,
            Self::UNSPECIFIED(v) => v,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize)]
#[repr(u8)]
enum EI_OSABI {
    ELFOSABI_SYSV = 0, // System V ABI
    ELFOSABI_HPUX = 1, // HP-UX operating system
    UNSPECIFIED(u8),
    ELFOSABI_STANDALONE = 255, // Standalone (embedded) application
}

impl EI_OSABI {
    pub fn parse(raw: &[u8]) -> IResult<&[u8], Self> {
        let (r, c) = number::complete::le_u8(raw)?;
        Ok((r, Self::from(c)))
    }
}

impl From<u8> for EI_OSABI {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::ELFOSABI_SYSV,
            1 => Self::ELFOSABI_HPUX,
            255 => Self::ELFOSABI_STANDALONE,
            _ => Self::UNSPECIFIED(value),
        }
    }
}

impl Into<u8> for EI_OSABI {
    fn into(self) -> u8 {
        match self {
            Self::ELFOSABI_HPUX => 1,
            Self::ELFOSABI_STANDALONE => 255,
            Self::ELFOSABI_SYSV => 0,
            Self::UNSPECIFIED(v) => v,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct EIdent {
    EI_MAGIC: [u8; 4],  // File identification [0x7f, E, L, F]
    EI_CLASS: EI_CLASS, // 4 File class
    EI_DATA: EI_DATA,   // 5 Data encoding
    EI_VERSION: u8,     // 6 File version
    EI_OSABI: EI_OSABI, // 7 OS/ABI identification
    EI_ABIVERSION: u8,  // 8 ABI version
    EI_PAD: u8,         // 9 Start of padding bytes
    UNSPECIFIED: [u8; 5],
    EI_NIDENT: u8, // 16 Size of e_ident[]
}

impl EIdent {
    pub fn new() -> Self {
        Self {
            EI_MAGIC: [0x7f, 69, 76, 70],
            EI_DATA: EI_DATA::ELFDATA2LSB,
            EI_CLASS: EI_CLASS::ELFCLASS64,
            EI_OSABI: EI_OSABI::ELFOSABI_SYSV,
            EI_ABIVERSION: 0,
            EI_VERSION: 1,
            EI_PAD: 0,
            EI_NIDENT: 0,
            UNSPECIFIED: [0; 5],
        }
    }
    pub fn parse(raw: &[u8]) -> IResult<&[u8], Self> {
        let (remaining, _magic) = tag([0x7f, 69, 76, 70])(raw)?;
        let (remaining, class) = EI_CLASS::parse(remaining)?;
        let (remaining, data) = EI_DATA::parse(remaining)?;
        let (remaining, version) = number::complete::le_u8(remaining)?;
        let (remaining, (osabi, abi_version)) =
            sequence::tuple((EI_OSABI::parse, number::complete::le_u8))(remaining)?;
        let (remaining, pad) = number::complete::le_u8(remaining)?;
        let (remaining, unspecified) = take(5usize)(remaining)?;
        let (remaining, nident) = number::complete::le_u8(remaining)?;
        Ok((
            remaining,
            Self {
                EI_MAGIC: [0x7f, 69, 76, 70],
                EI_CLASS: class,
                EI_DATA: data,
                EI_VERSION: version,
                EI_PAD: pad,
                UNSPECIFIED: unspecified[0..5].try_into().unwrap(),
                EI_NIDENT: nident,
                EI_ABIVERSION: abi_version,
                EI_OSABI: osabi,
            },
        ))
    }
}

impl Into<Vec<u8>> for EIdent {
    fn into(self) -> Vec<u8> {
        let mut out: Vec<u8> = Vec::new();
        out.extend(self.EI_MAGIC);
        out.push(self.EI_CLASS.into());
        out.push(self.EI_DATA.into());
        out.push(self.EI_VERSION.into());
        out.push(self.EI_OSABI.into());
        out.push(self.EI_ABIVERSION.into());
        out.push(self.EI_PAD);
        out.extend(self.UNSPECIFIED);
        out.push(self.EI_NIDENT);
        out
    }
}

impl Into<Vec<u8>> for &EIdent {
    fn into(self) -> Vec<u8> {
        let mut out: Vec<u8> = Vec::new();
        out.extend(self.EI_MAGIC);
        out.push(self.EI_CLASS.into());
        out.push(self.EI_DATA.into());
        out.push(self.EI_VERSION.into());
        out.push(self.EI_OSABI.into());
        out.push(self.EI_ABIVERSION.into());
        out.push(self.EI_PAD);
        out.extend(self.UNSPECIFIED);
        out.push(self.EI_NIDENT);
        out
    }
}
