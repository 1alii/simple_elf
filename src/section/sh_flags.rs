#![allow(non_camel_case_types)]

use enumflags2::bitflags;
use serde::Serialize;
use std::fmt::Debug;

#[bitflags]
#[derive(Clone, Copy, Debug, Serialize)]
#[repr(u64)]
pub enum SHFlags {
    SHF_WRITE = 0x1,
    SHF_ALLOC = 0x2,
    SHF_EXECINSTR = 0x4,
    SHF_MERGE = 0x10,
    SHF_STRINGS = 0x20,
    SHF_INFO_LINK = 0x40,
    SHF_LINK_ORDER = 0x80,
    SHF_OS_NONCONFORMING = 0x100,
    SHF_GROUP = 0x200,
    SHF_TLS = 0x400,
    SHF_COMPRESSED = 0x800,
    SHF_X86_64_LASHFlagsRGE = 0x10000000,
}
