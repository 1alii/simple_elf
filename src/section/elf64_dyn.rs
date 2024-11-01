#![allow(non_camel_case_types)]

use nom::number;
use nom::IResult;

#[derive(Debug, Clone, Copy)]
#[repr(u64)]
pub enum DynamicTag {
    DT_NULL = 0,
    DT_NEEDED = 1,
    DT_PLTRELSZ = 2,
    DT_PLTGOT = 3,
    DT_HASH = 4,
    DT_STRTAB = 5,
    DT_SYMTAB = 6,
    DT_RELA = 7,
    DT_RELASZ = 8,
    DT_RELAENT = 9,
    DT_STRSZ = 10,
    DT_SYMENT = 11,
    DT_INIT = 12,
    DT_FINI = 13,
    DT_SONAME = 14,
    DT_RPATH = 15,
    DT_SYMBOLIC = 16,
    DT_REL = 17,
    DT_RELSZ = 18,
    DT_RELENT = 19,
    DT_PLTREL = 20,
    DT_DEBUG = 21,
    DT_TEXTREL = 22,
    DT_JMPREL = 23,
    DT_BIND_NOW = 24,
    DT_INIT_ARRAY = 25,
    DT_FINI_ARRAY = 26,
    DT_INIT_ARRAYSZ = 27,
    DT_FINI_ARRAYSZ = 28,
    DT_RUN_PATH = 0x1d,
    DT_GNU_HASH = 0x6ffffef5,
    DT_Flags1 = 0x6ffffffb,
    DT_RELA_COUNT = 0x6ffffff9,
    DT_X86_64_PLT = 0x70000000,
    DT_X86_64_PLTSZ = 0x70000001,
    DT_X86_64_PLTENT = 0x70000003,
    UNSPECIFIED(u64),
}

impl Into<u64> for DynamicTag {
    fn into(self) -> u64 {
        match self {
            Self::DT_NULL => 0,
            Self::DT_NEEDED => 1,
            Self::DT_PLTRELSZ => 2,
            Self::DT_PLTGOT => 3,
            Self::DT_HASH => 4,
            Self::DT_STRTAB => 5,
            Self::DT_SYMTAB => 6,
            Self::DT_RELA => 7,
            Self::DT_RELASZ => 8,
            Self::DT_RELAENT => 9,
            Self::DT_STRSZ => 10,
            Self::DT_SYMENT => 11,
            Self::DT_INIT => 12,
            Self::DT_FINI => 13,
            Self::DT_SONAME => 14,
            Self::DT_RPATH => 15,
            Self::DT_SYMBOLIC => 16,
            Self::DT_REL => 17,
            Self::DT_RELSZ => 18,
            Self::DT_RELENT => 19,
            Self::DT_PLTREL => 20,
            Self::DT_DEBUG => 21,
            Self::DT_TEXTREL => 22,
            Self::DT_JMPREL => 23,
            Self::DT_BIND_NOW => 24,
            Self::DT_INIT_ARRAY => 25,
            Self::DT_FINI_ARRAY => 26,
            Self::DT_INIT_ARRAYSZ => 27,
            Self::DT_FINI_ARRAYSZ => 28,
            Self::DT_RUN_PATH => 0x1d,
            Self::DT_GNU_HASH => 0x6ffffef5,
            Self::DT_Flags1 => 0x6ffffffb,
            Self::DT_RELA_COUNT => 0x6ffffff9,
            Self::DT_X86_64_PLT => 0x70000000,
            Self::DT_X86_64_PLTSZ => 0x70000001,
            Self::DT_X86_64_PLTENT => 0x70000003,
            Self::UNSPECIFIED(v) => v,
        }
    }
}

impl From<u64> for DynamicTag {
    fn from(value: u64) -> Self {
        match value {
            0 => Self::DT_NULL,
            1 => Self::DT_NEEDED,
            2 => Self::DT_PLTRELSZ,
            3 => Self::DT_PLTGOT,
            4 => Self::DT_HASH,
            5 => Self::DT_STRTAB,
            6 => Self::DT_SYMTAB,
            7 => Self::DT_RELA,
            8 => Self::DT_RELASZ,
            9 => Self::DT_RELAENT,
            10 => Self::DT_STRSZ,
            11 => Self::DT_SYMENT,
            12 => Self::DT_INIT,
            13 => Self::DT_FINI,
            14 => Self::DT_SONAME,
            15 => Self::DT_RPATH,
            16 => Self::DT_SYMBOLIC,
            17 => Self::DT_REL,
            18 => Self::DT_RELSZ,
            19 => Self::DT_RELENT,
            20 => Self::DT_PLTREL,
            21 => Self::DT_DEBUG,
            22 => Self::DT_TEXTREL,
            23 => Self::DT_JMPREL,
            24 => Self::DT_BIND_NOW,
            25 => Self::DT_INIT_ARRAY,
            26 => Self::DT_FINI_ARRAY,
            27 => Self::DT_INIT_ARRAYSZ,
            28 => Self::DT_FINI_ARRAYSZ,
            0x1d => Self::DT_RUN_PATH,
            0x6ffffef5 => Self::DT_GNU_HASH,
            0x6ffffffb => Self::DT_Flags1,
            0x6ffffff9 => Self::DT_RELA_COUNT,
            0x70000000 => Self::DT_X86_64_PLT,
            0x70000001 => Self::DT_X86_64_PLTSZ,
            0x70000003 => Self::DT_X86_64_PLTENT,
            _ => Self::UNSPECIFIED(value),
        }
    }
}

pub struct Elf64_Dyn {
    pub d_tag: DynamicTag,
    pub val_ptr: u64,
}

impl Elf64_Dyn {
    pub fn parse(raw: &[u8]) -> IResult<&[u8], Self> {
        let (remaining, d_tag) = number::complete::le_u64(raw)?;
        let (remaining, val_ptr) = number::complete::le_u64(remaining)?;
        Ok((
            remaining,
            Self {
                d_tag: d_tag.into(),
                val_ptr,
            },
        ))
    }
}
