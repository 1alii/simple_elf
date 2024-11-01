#![allow(non_camel_case_types)]

use super::sh_flags::SHFlags;
use super::sh_type::ShType;

use enumflags2::BitFlags;
use nom::number::complete::{le_u32, le_u64};
use nom::sequence;
use nom::IResult;
use serde::Serialize;
use std::fmt::Debug;

#[derive(Debug, Serialize)]
pub struct Elf64Shdr {
    pub sh_name: u32,                /* Section name */
    pub sh_type: ShType,             /* Section type */
    pub sh_flags: BitFlags<SHFlags>, /* Section attributes */
    pub sh_addr: u64,                /* Virtual address in memory */
    pub sh_offset: u64,              /* Offset in file */
    pub sh_size: u64,                /* Size of section */
    pub sh_link: u32,                /* Link to other section */
    pub sh_info: u32,                /* Miscellaneous information */
    pub sh_addralign: u64,           /* Address alignment boundary */
    pub sh_entsize: u64,             /* Size of entries, if section has table */
}

impl Elf64Shdr {
    pub fn parse(raw: &[u8]) -> IResult<&[u8], Self> {
        let (
            r,
            (
                sh_name,
                sh_type,
                sh_flags,
                sh_addr,
                sh_offset,
                sh_size,
                sh_link,
                sh_info,
                sh_addralign,
                sh_entsize,
            ),
        ) = sequence::tuple((
            le_u32,
            ShType::parse,
            le_u64,
            le_u64,
            le_u64,
            le_u64,
            le_u32,
            le_u32,
            le_u64,
            le_u64,
        ))(raw)?;
        Ok((
            r,
            Self {
                sh_name,
                sh_type,
                sh_flags: BitFlags::<SHFlags>::from_bits(sh_flags).unwrap(),
                sh_addr,
                sh_offset,
                sh_size,
                sh_link,
                sh_info,
                sh_addralign,
                sh_entsize,
            },
        ))
    }
}

impl Into<Vec<u8>> for Elf64Shdr {
    fn into(self) -> Vec<u8> {
        let mut out: Vec<u8> = Vec::new();
        out.extend(self.sh_name.to_le_bytes());
        out.extend::<Vec<u8>>(self.sh_type.into());
        out.extend(self.sh_flags.bits().to_le_bytes());
        out.extend(self.sh_addr.to_le_bytes());
        out.extend(self.sh_offset.to_le_bytes());
        out.extend(self.sh_size.to_le_bytes());
        out.extend(self.sh_link.to_le_bytes());
        out.extend(self.sh_info.to_le_bytes());
        out.extend(self.sh_addralign.to_le_bytes());
        out.extend(self.sh_entsize.to_le_bytes());
        out
    }
}

impl Into<Vec<u8>> for &Elf64Shdr {
    fn into(self) -> Vec<u8> {
        let mut out: Vec<u8> = Vec::new();
        out.extend(self.sh_name.to_le_bytes());
        out.extend::<Vec<u8>>(self.sh_type.into());
        out.extend(self.sh_flags.bits().to_le_bytes());
        out.extend(self.sh_addr.to_le_bytes());
        out.extend(self.sh_offset.to_le_bytes());
        out.extend(self.sh_size.to_le_bytes());
        out.extend(self.sh_link.to_le_bytes());
        out.extend(self.sh_info.to_le_bytes());
        out.extend(self.sh_addralign.to_le_bytes());
        out.extend(self.sh_entsize.to_le_bytes());
        out
    }
}
