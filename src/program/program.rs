#![allow(non_camel_case_types)]

use super::super::common::{ParserIn, ParserOut, RawBinaryData};
use super::program_header::Elf64_Phdr;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Program {
    pub header: Elf64_Phdr,
    pub data: RawBinaryData,
}

impl Program {
    pub fn parse(input: ParserIn) -> ParserOut<Self> {
        let (r, header) = Elf64_Phdr::parse(input.remaining)?;
        Ok((
            (input.whole_file, r).into(),
            Self {
                data: input.whole_file
                    [header.p_offset as usize..(header.p_offset + header.p_filesz) as usize]
                    .into(),
                header,
            },
        ))
    }
}
