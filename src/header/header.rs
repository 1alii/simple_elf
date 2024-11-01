#![allow(non_camel_case_types)]

use super::e_ident::EIdent;
use super::e_machine::EMachine;
use super::e_type::EType;
use nom::number;
use nom::number::complete;
use nom::sequence;
use nom::IResult;
use serde::Serialize;

#[derive(Debug, Clone, Copy, Serialize)]
#[repr(u32)]
pub enum EVersion {
    EV_NONE = 0, // Invalid versionn
    EV_CURRENT = 1,
    UNSPECIFIED(u32),
}

impl EVersion {
    pub fn parse(raw: &[u8]) -> IResult<&[u8], Self> {
        let (r, c) = number::complete::le_u32(raw)?;
        Ok((r, Self::from(c)))
    }
}

impl From<u32> for EVersion {
    fn from(value: u32) -> Self {
        match value {
            0 => Self::EV_NONE,
            1 => Self::EV_CURRENT,
            v => Self::UNSPECIFIED(v),
        }
    }
}

impl Into<u32> for EVersion {
    fn into(self) -> u32 {
        match self {
            Self::EV_CURRENT => 1,
            Self::EV_NONE => 0,
            Self::UNSPECIFIED(v) => v,
        }
    }
}

impl Into<Vec<u8>> for EVersion {
    fn into(self) -> Vec<u8> {
        let tmp: u32 = self.into();
        tmp.to_le_bytes().into()
    }
}

impl Into<Vec<u8>> for &EVersion {
    fn into(self) -> Vec<u8> {
        let tmp: u32 = (*self).into();
        tmp.to_le_bytes().into()
    }
}

#[derive(Debug, Serialize)]
pub struct Elf64Ehdr {
    pub e_ident: EIdent,     /* ELF identification */
    pub e_type: EType,       /* Object file type */
    pub e_machine: EMachine, /* Machine type */
    pub e_version: EVersion, /* Object file version */
    pub e_entry: u64,        /* Entry point address */
    pub e_phoff: u64,        /* Program header offset */
    pub e_shoff: u64,        /* Section header offset */
    pub e_flags: u32,        /* Processor-specific flags */
    pub e_ehsize: u16,       /* ELF header size */
    pub e_phentsize: u16,    /* Size of program header entry */
    pub e_phnum: u16,        /* Number of program header entries */
    pub e_shentsize: u16,    /* Size of section header entry */
    pub e_shnum: u16,        /* Number of section header entries */
    pub e_shstrndx: u16,     /* Section name string table index */
}

impl Elf64Ehdr {
    pub fn new(file_type: EType) -> Self {
        Self {
            e_type: file_type,               // set by user
            e_machine: EMachine::EM_X86_64,  // const
            e_version: EVersion::EV_CURRENT, // const
            e_entry: 0,                      // set by user
            e_flags: 0,                      // set by user
            e_phnum: 0,                      // set when packing
            e_shnum: 0,                      // set when packing
            e_phoff: 0,                      // set when packing
            e_shoff: 0,                      // set when packing
            e_ident: EIdent::new(),          // const
            e_ehsize: 64,                    // const
            e_shstrndx: 0,                   // set by user
            e_phentsize: 56,                 // const
            e_shentsize: 64,                 // const
        }
    }
    pub fn parse(raw: &[u8]) -> IResult<&[u8], Self> {
        let (remaining, ident) = EIdent::parse(raw)?;
        let (remaining, (e_type, e_machine)) =
            sequence::tuple((EType::parse, EMachine::parse))(remaining)?;
        let (remaining, version) = EVersion::parse(remaining)?;
        let (remaining, (entry, phoff, shoff)) =
            sequence::tuple((complete::le_u64, complete::le_u64, complete::le_u64))(remaining)?;
        let (remaining, flags) = number::complete::le_u32(remaining)?;
        let (remaining, (e_ehsize, e_phentsize, e_phnum, e_shentsize, e_shnum, e_shstrndx)) =
            sequence::tuple((
                complete::le_u16,
                complete::le_u16,
                complete::le_u16,
                complete::le_u16,
                complete::le_u16,
                complete::le_u16,
            ))(remaining)?;
        Ok((
            remaining,
            Self {
                e_ident: ident,
                e_type,
                e_machine,
                e_version: version,
                e_entry: entry,
                e_phoff: phoff,
                e_shoff: shoff,
                e_ehsize,
                e_phentsize,
                e_phnum,
                e_shentsize,
                e_shnum,
                e_shstrndx,
                e_flags: flags,
            },
        ))
    }
}

impl Into<Vec<u8>> for Elf64Ehdr {
    fn into(self) -> Vec<u8> {
        let mut out: Vec<u8> = Vec::new();
        out.extend::<Vec<u8>>(self.e_ident.into());
        out.extend::<Vec<u8>>(self.e_type.into());
        out.extend::<Vec<u8>>(self.e_machine.into());
        out.extend::<Vec<u8>>(self.e_version.into());
        out.extend::<[u8; 8]>(self.e_entry.to_le_bytes());
        out.extend::<[u8; 8]>(self.e_phoff.to_le_bytes());
        out.extend::<[u8; 8]>(self.e_shoff.to_le_bytes());
        out.extend::<[u8; 4]>(self.e_flags.to_le_bytes());
        out.extend::<[u8; 2]>(self.e_ehsize.to_le_bytes());
        out.extend::<[u8; 2]>(self.e_phentsize.to_le_bytes());
        out.extend::<[u8; 2]>(self.e_phnum.to_le_bytes());
        out.extend::<[u8; 2]>(self.e_shentsize.to_le_bytes());
        out.extend::<[u8; 2]>(self.e_shnum.to_le_bytes());
        out.extend::<[u8; 2]>(self.e_shstrndx.to_le_bytes());
        out
    }
}

impl Into<Vec<u8>> for &Elf64Ehdr {
    fn into(self) -> Vec<u8> {
        let mut out: Vec<u8> = Vec::new();
        out.extend::<Vec<u8>>(self.e_ident.clone().into());
        out.extend::<Vec<u8>>(self.e_type.into());
        out.extend::<Vec<u8>>(self.e_machine.into());
        out.extend::<Vec<u8>>(self.e_version.into());
        out.extend::<[u8; 8]>(self.e_entry.to_le_bytes());
        out.extend::<[u8; 8]>(self.e_phoff.to_le_bytes());
        out.extend::<[u8; 8]>(self.e_shoff.to_le_bytes());
        out.extend::<[u8; 4]>(self.e_flags.to_le_bytes());
        out.extend::<[u8; 2]>(self.e_ehsize.to_le_bytes());
        out.extend::<[u8; 2]>(self.e_phentsize.to_le_bytes());
        out.extend::<[u8; 2]>(self.e_phnum.to_le_bytes());
        out.extend::<[u8; 2]>(self.e_shentsize.to_le_bytes());
        out.extend::<[u8; 2]>(self.e_shnum.to_le_bytes());
        out.extend::<[u8; 2]>(self.e_shstrndx.to_le_bytes());
        out
    }
}
