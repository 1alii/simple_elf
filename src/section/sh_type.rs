#![allow(non_camel_case_types)]

use nom::number::complete::le_u32;
use nom::IResult;
use serde::Serialize;

use std::fmt::Debug;

#[derive(Debug, Clone, Copy, Serialize)]
#[repr(u32)]
pub enum ShType {
    /**
     * 0x60000000 to 0x6FFFFFFF Environment-specific use
     * 0x70000000 to 0x7FFFFFFF Processor-specific use
     */
    SHT_NULL = 0, // Marks an unused section header
    SHT_PROGBITS = 1, // Contains information defined by the program
    SHT_SYMTAB = 2,   // Contains a linker symbol table
    SHT_STRTAB = 3,   // Contains a string table
    SHT_RELA = 4,     // Contains “Rela” type relocation entries
    SHT_HASH = 5,     // Contains a symbol hash table
    SHT_DYNAMIC = 6,  // Contains dynamic linking tables
    SHT_NOTE = 7,     // Contains note information
    SHT_NOBITS = 8,   // Contains uninitialized space; does not occupy any space in the file
    SHT_REL = 9,      // Contains “Rel” type relocation entries
    SHT_SHLIB = 10,   // Reserved
    SHT_DYNSYM = 11,  // Contains a dynamic loader symbol table
    SHT_INIT_ARRAY = 0xe,
    SHT_FINI_ARRAY = 0xf,
    SHT_PREINIT_ARRAY = 0x10,
    SHT_GNU_liblist = 0x6ffffff5,
    SHT_GNU_hash = 0x6ffffff6,
    SHT_GNU_attributes = 0x6ffffff7,
    SHT_GNU_verdef = 0x6ffffffd,
    SHT_GNU_verneed = 0x6ffffffe,
    SHT_GNU_versym = 0x6fffffff,
    SHT_X86_64_UNWIND = 0x70000001,
    UNSPECIFIED(u32),
}

impl Into<Vec<u8>> for ShType {
    fn into(self) -> Vec<u8> {
        let tmp: u32 = self.into();
        tmp.to_le_bytes().into()
    }
}

impl ShType {
    pub fn parse(raw: &[u8]) -> IResult<&[u8], Self> {
        let (r, c) = le_u32(raw)?;
        Ok((r, Self::from(c)))
    }
}

impl From<u32> for ShType {
    fn from(value: u32) -> Self {
        match value {
            0 => Self::SHT_NULL,
            1 => Self::SHT_PROGBITS,
            2 => Self::SHT_SYMTAB,
            3 => Self::SHT_STRTAB,
            4 => Self::SHT_RELA,
            5 => Self::SHT_HASH,
            6 => Self::SHT_DYNAMIC,
            7 => Self::SHT_NOTE,
            8 => Self::SHT_NOBITS,
            9 => Self::SHT_REL,
            10 => Self::SHT_SHLIB,
            11 => Self::SHT_DYNSYM,
            0xe => Self::SHT_INIT_ARRAY,
            0xf => Self::SHT_FINI_ARRAY,
            0x10 => Self::SHT_PREINIT_ARRAY,
            0x6ffffff5 => Self::SHT_GNU_liblist,
            0x6ffffff6 => Self::SHT_GNU_hash,
            0x6ffffff7 => Self::SHT_GNU_attributes,
            0x6ffffffd => Self::SHT_GNU_verdef,
            0x6ffffffe => Self::SHT_GNU_verneed,
            0x6fffffff => Self::SHT_GNU_versym,
            0x70000001 => Self::SHT_X86_64_UNWIND,
            _ => Self::UNSPECIFIED(value),
        }
    }
}

impl Into<u32> for ShType {
    fn into(self) -> u32 {
        match self {
            Self::UNSPECIFIED(v) => v,
            Self::SHT_DYNAMIC => 6,
            Self::SHT_DYNSYM => 11,
            Self::SHT_HASH => 5,
            Self::SHT_NOBITS => 8,
            Self::SHT_NOTE => 7,
            Self::SHT_NULL => 0,
            Self::SHT_PROGBITS => 1,
            Self::SHT_REL => 9,
            Self::SHT_RELA => 4,
            Self::SHT_SHLIB => 10,
            Self::SHT_STRTAB => 3,
            Self::SHT_SYMTAB => 2,
            Self::SHT_FINI_ARRAY => 0xf,
            Self::SHT_INIT_ARRAY => 0xe,
            Self::SHT_GNU_verdef => 0x6ffffffd,
            Self::SHT_PREINIT_ARRAY => 0x10,
            Self::SHT_GNU_liblist => 0x6ffffff5,
            Self::SHT_GNU_hash => 0x6ffffff6,
            Self::SHT_GNU_attributes => 0x6ffffff7,
            Self::SHT_GNU_verneed => 0x6ffffffe,
            Self::SHT_GNU_versym => 0x6fffffff,
            Self::SHT_X86_64_UNWIND => 0x70000001,
        }
    }
}
