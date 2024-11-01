#![allow(non_camel_case_types)]

use std::collections::HashMap;

use super::common::ParserIn;
use super::header::header::Elf64Ehdr;
use super::program::program::Program;
use super::section::section::{Section, SectionData};
use nom::multi::count;
use nom::IResult;
use serde::Serialize;
use serde_json;

#[derive(Debug, Serialize)]
pub struct Elf64 {
    pub header: Elf64Ehdr,
    pub sections: Vec<Section>,
    pub programs: Vec<Program>,
}

const PN_XNUM: u16 = 0xFFFF;

impl Elf64 {
    pub fn json_report(&self) -> String {
        serde_json::to_string_pretty(self).unwrap()
    }
    pub fn parse(raw: &[u8]) -> IResult<&[u8], Self> {
        let (remaining, header) = Elf64Ehdr::parse(raw)?;
        let (_, mut sections) = count(Section::parse, header.e_shnum as usize)(ParserIn::from((
            raw,
            &raw[header.e_shoff as usize..],
        )))?;
        let mut section_names: Option<HashMap<usize, String>> = None;
        if sections.len() > header.e_shstrndx as usize {
            // if let SectionData::String(s) = sections[header.e_shstrndx as usize].data.clone() {
            //     section_names = Some(&s.strings);
            // }
            match &sections[header.e_shstrndx as usize].data {
                SectionData::String(s) => {
                    section_names = Some(s.strings.clone());
                }
                _ => {}
            }
        }

        for section in &mut sections {
            match &section_names {
                Some(sn) => {
                    if let Some(v) = sn.get(&(section.header.sh_name as usize)) {
                        section.name = v.clone();
                    }
                }
                _ => {}
            }
            // todo resolve symbol names from strtab
        }
        let mut number_of_headers: u32 = header.e_phnum as u32;
        if header.e_phnum == PN_XNUM {
            /*
             * if value of phnum is 0xFFFF number of program headers are in the
             * sh_info field of the first section
             */
            number_of_headers = sections[0].header.sh_info;
        }
        let (_, programs) = count(Program::parse, number_of_headers as usize)(
            (raw, &raw[header.e_phoff as usize..]).into(),
        )?;
        Ok((
            remaining,
            Self {
                header,
                sections,
                programs,
            },
        ))
    }
}

impl<T: AsRef<[u8]>> From<&T> for Elf64 {
    fn from(value: &T) -> Self {
        let raw = value.as_ref();
        let f = Self::parse(raw).unwrap();
        f.1
    }
}
