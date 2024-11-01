#![allow(non_camel_case_types)]
use super::p_type::PType;
use enumflags2::{bitflags, BitFlags};
use nom::number::complete::{le_u32, le_u64};
use nom::sequence;
use nom::IResult;
use serde::Serialize;

#[bitflags]
#[derive(Clone, Copy, Debug, Serialize)]
#[repr(u32)]
pub enum PFlags {
    Execute = 0x1,
    Write = 0x2,
    Read = 0x4,
}

#[derive(Debug, Serialize)]
pub struct Elf64_Phdr {
    pub p_type: PType,             /* Type of segment */
    pub p_flags: BitFlags<PFlags>, /* Segment attributes */
    pub p_offset: u64,             /* Offset in file :: set when packing*/
    pub p_vaddr: u64,              /* Virtual address in memory */
    pub p_paddr: u64,              /* Reserved */
    pub p_filesz: u64,             /* Size of segment in file :: set when packing*/
    pub p_memsz: u64,              /* Size of segment in memory */
    pub p_align: u64,              /* Alignment of segment */
}

impl Elf64_Phdr {
    pub fn memory_mapping(&self) -> String {
        format!(
            "{}-{} perm: {}",
            self.p_vaddr,
            self.p_vaddr + self.p_memsz,
            self.p_flags
        )
    }
    pub fn parse(raw: &[u8]) -> IResult<&[u8], Self> {
        let (r, (p_type, p_flags, p_offset, p_vaddr, p_paddr, p_filesz, p_memsz, p_align)) =
            sequence::tuple((
                PType::parse,
                le_u32,
                le_u64,
                le_u64,
                le_u64,
                le_u64,
                le_u64,
                le_u64,
            ))(raw)?;

        Ok((
            r,
            Self {
                p_type,
                p_flags: BitFlags::<PFlags>::from_bits(p_flags).unwrap(),
                p_offset,
                p_vaddr,
                p_paddr,
                p_filesz,
                p_memsz,
                p_align,
            },
        ))
    }
}

impl Into<Vec<u8>> for Elf64_Phdr {
    fn into(self) -> Vec<u8> {
        let mut out: Vec<u8> = Vec::new();
        out.extend::<Vec<u8>>(self.p_type.into());
        out.extend(self.p_flags.bits().to_le_bytes());
        out.extend(self.p_offset.to_le_bytes());
        out.extend(self.p_vaddr.to_le_bytes());
        out.extend(self.p_paddr.to_le_bytes());
        out.extend(self.p_filesz.to_le_bytes());
        out.extend(self.p_memsz.to_le_bytes());
        out.extend(self.p_align.to_le_bytes());
        out
    }
}

impl Into<Vec<u8>> for &Elf64_Phdr {
    fn into(self) -> Vec<u8> {
        let mut out: Vec<u8> = Vec::new();
        out.extend::<Vec<u8>>(self.p_type.into());
        out.extend(self.p_flags.bits().to_le_bytes());
        out.extend(self.p_offset.to_le_bytes());
        out.extend(self.p_vaddr.to_le_bytes());
        out.extend(self.p_paddr.to_le_bytes());
        out.extend(self.p_filesz.to_le_bytes());
        out.extend(self.p_memsz.to_le_bytes());
        out.extend(self.p_align.to_le_bytes());
        out
    }
}
