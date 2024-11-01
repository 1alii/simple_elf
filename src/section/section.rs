use super::super::common::{ParserIn, ParserOut, RawBinaryData};
use super::section_header::Elf64Shdr;
use super::sh_type::ShType;
use super::symbol::{Elf64_Rel, Elf64_Rela, Elf64_Sym};
use nom::multi;
use serde::ser::SerializeSeq;
use serde::Serialize;
use std::collections::HashMap;
use std::ffi::{CStr, CString};

#[derive(Debug, Serialize)]
pub struct Section {
    pub header: Elf64Shdr,
    pub name: String, // this is the name of this table pointed by sh_name field to the string table section
    pub data: SectionData,
}

impl Section {
    pub fn parse(input: ParserIn<'_>) -> ParserOut<Self> {
        let (r, header) = Elf64Shdr::parse(input.remaining)?;
        let mut raw_data: Option<&[u8]> = None;
        if header.sh_offset != 0 && header.sh_size != 0 {
            raw_data = Some(
                (&input.whole_file
                    [header.sh_offset as usize..(header.sh_offset + header.sh_size) as usize])
                    .into(),
            );
        }
        match header.sh_type {
            ShType::SHT_NOBITS => {
                return Ok((
                    (input.whole_file, r).into(),
                    Self {
                        name: String::new(),
                        header,
                        data: SectionData::Unknown(Vec::new().into()),
                    },
                ))
            }
            ShType::SHT_DYNSYM | ShType::SHT_SYMTAB => {
                if let Some(raw) = raw_data {
                    return Ok((
                        (input.whole_file, r).into(),
                        Self {
                            name: String::new(),
                            header,
                            data: SectionData::Symbol(raw.into()),
                        },
                    ));
                }
                return Ok((
                    (input.whole_file, r).into(),
                    Self {
                        name: String::new(),
                        header,
                        data: SectionData::Symbol(SymbolSection::default()),
                    },
                ));
            }
            ShType::SHT_STRTAB => {
                if let Some(raw) = raw_data {
                    return Ok((
                        (input.whole_file, r).into(),
                        Self {
                            name: String::new(),
                            header,
                            data: SectionData::String(raw.into()),
                        },
                    ));
                }
                return Ok((
                    (input.whole_file, r).into(),
                    Self {
                        name: String::new(),
                        header,
                        data: SectionData::String(StringSection::default()),
                    },
                ));
            }
            ShType::SHT_REL => {
                if let Some(raw) = raw_data {
                    return Ok((
                        (input.whole_file, r).into(),
                        Self {
                            name: String::new(),
                            header,
                            data: SectionData::Rel(raw.into()),
                        },
                    ));
                }
                return Ok((
                    (input.whole_file, r).into(),
                    Self {
                        name: String::new(),
                        header,
                        data: SectionData::Rel(RelSection::default()),
                    },
                ));
            }
            ShType::SHT_RELA => {
                if let Some(raw) = raw_data {
                    return Ok((
                        (input.whole_file, r).into(),
                        Self {
                            name: String::new(),
                            header,
                            data: SectionData::Rela(raw.into()),
                        },
                    ));
                }
                return Ok((
                    (input.whole_file, r).into(),
                    Self {
                        name: String::new(),
                        header,
                        data: SectionData::Rela(RelaSection::default()),
                    },
                ));
            }
            _ => {
                if let Some(raw) = raw_data {
                    return Ok((
                        (input.whole_file, r).into(),
                        Self {
                            name: String::new(),
                            header,
                            data: SectionData::Unknown(raw.into()),
                        },
                    ));
                }
                return Ok((
                    (input.whole_file, r).into(),
                    Self {
                        name: String::new(),
                        header,
                        data: SectionData::Unknown(UnImplementedSection::default()),
                    },
                ));
            }
        }
    }
}

#[derive(Debug, Serialize)]
pub enum SectionData {
    String(StringSection),
    Rela(RelaSection),
    Rel(RelSection),
    Symbol(SymbolSection),
    Unknown(UnImplementedSection),
}

impl Into<Vec<u8>> for SectionData {
    fn into(self) -> Vec<u8> {
        match self {
            Self::String(s) => s.into(),
            Self::Rel(s) => s.into(),
            Self::Rela(s) => s.into(),
            Self::Symbol(s) => s.into(),
            Self::Unknown(s) => s.data.into(),
        }
    }
}

impl Into<Vec<u8>> for &SectionData {
    fn into(self) -> Vec<u8> {
        match &self {
            SectionData::String(s) => s.into(),
            SectionData::Rel(s) => s.into(),
            SectionData::Rela(s) => s.into(),
            SectionData::Symbol(s) => s.into(),
            SectionData::Unknown(s) => s.into(),
        }
    }
}

#[derive(Default, Debug, Serialize)]
pub struct UnImplementedSection {
    pub data: RawBinaryData,
}

impl Into<Vec<u8>> for UnImplementedSection {
    fn into(self) -> Vec<u8> {
        self.data.into()
    }
}

impl Into<Vec<u8>> for &UnImplementedSection {
    fn into(self) -> Vec<u8> {
        (&self.data).into()
    }
}

impl<T: AsRef<[u8]>> From<T> for UnImplementedSection {
    fn from(value: T) -> Self {
        Self { data: value.into() }
    }
}

#[derive(Default, Debug)]
pub struct StringSection {
    pub strings: HashMap<usize, String>,
}

impl Serialize for StringSection {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.strings.len()))?;
        for e in self.strings.values() {
            seq.serialize_element(e)?;
        }
        seq.end()
    }
}

impl<T: AsRef<[u8]>> From<T> for StringSection {
    fn from(value: T) -> Self {
        let v = value.as_ref();
        let mut x: HashMap<usize, String> = HashMap::new();
        let mut end_of_last_str: usize = 0;
        loop {
            if let Ok(g) = CStr::from_bytes_until_nul(&v[end_of_last_str..]) {
                let string_from_cstr = g.to_string_lossy().into_owned();
                x.insert(end_of_last_str, string_from_cstr);
                end_of_last_str += g.to_bytes().len() + 1;
            } else {
                break;
            }
        }
        Self { strings: x }
    }
}

impl Into<Vec<u8>> for StringSection {
    fn into(self) -> Vec<u8> {
        let mut out: Vec<u8> = Vec::new();
        let mut strings = self.strings.iter().collect::<Vec<(&usize, &String)>>();
        strings.sort_by(|a, b| a.0.cmp(b.0));
        println!("owned:: strings are {:?}", strings);
        out.extend(
            strings
                .iter()
                .map(|a| a.1)
                .map(|s| CString::new(s.as_str()).unwrap())
                .flat_map(|a| Vec::from(a.as_bytes_with_nul())),
        );
        out
    }
}

impl Into<Vec<u8>> for &StringSection {
    fn into(self) -> Vec<u8> {
        let mut out: Vec<u8> = Vec::new();
        let mut strings = self.strings.iter().collect::<Vec<(&usize, &String)>>();
        strings.sort_by(|a, b| a.0.cmp(b.0));
        println!("borrowed:: strings are {:?}", strings);
        out.extend(
            strings
                .iter()
                .map(|a| a.1)
                .map(|s| CString::new(s.as_str()).unwrap())
                .flat_map(|a| Vec::from(a.as_bytes_with_nul())),
        );
        out
    }
}

#[derive(Default, Debug, Serialize)]
pub struct RelaSection {
    pub rela_entries: Vec<Elf64_Rela>,
}

impl<T: AsRef<[u8]>> From<T> for RelaSection {
    fn from(value: T) -> Self {
        let v = value.as_ref();
        let (_, s) = multi::many0(Elf64_Rela::parse)(v).unwrap();
        Self { rela_entries: s }
    }
}

impl Into<Vec<u8>> for RelaSection {
    fn into(self) -> Vec<u8> {
        let mut out: Vec<u8> = Vec::new();
        out.extend(
            self.rela_entries
                .iter()
                .map::<Vec<u8>, _>(|a| a.into())
                .flat_map(|f| f),
        );
        out
    }
}

impl Into<Vec<u8>> for &RelaSection {
    fn into(self) -> Vec<u8> {
        let mut out: Vec<u8> = Vec::new();
        out.extend(
            self.rela_entries
                .iter()
                .map::<Vec<u8>, _>(|a| a.into())
                .flat_map(|f| f),
        );
        out
    }
}

#[derive(Default, Debug, Serialize)]
pub struct RelSection {
    pub rel_entries: Vec<Elf64_Rel>,
}

impl<T: AsRef<[u8]>> From<T> for RelSection {
    fn from(value: T) -> Self {
        let v = value.as_ref();
        let (_, s) = multi::many0(Elf64_Rel::parse)(v).unwrap();
        Self { rel_entries: s }
    }
}

impl Into<Vec<u8>> for RelSection {
    fn into(self) -> Vec<u8> {
        let mut out: Vec<u8> = Vec::new();
        out.extend(
            self.rel_entries
                .iter()
                .map::<Vec<u8>, _>(|a| a.into())
                .flat_map(|f| f),
        );
        out
    }
}

impl Into<Vec<u8>> for &RelSection {
    fn into(self) -> Vec<u8> {
        let mut out: Vec<u8> = Vec::new();
        out.extend(
            self.rel_entries
                .iter()
                .map::<Vec<u8>, _>(|a| a.into())
                .flat_map(|f| f),
        );
        out
    }
}

#[derive(Default, Debug, Serialize)]
pub struct SymbolSection {
    pub symbols: Vec<Elf64_Sym>,
}

impl<T: AsRef<[u8]>> From<T> for SymbolSection {
    fn from(value: T) -> Self {
        let v = value.as_ref();
        let (_, s) = multi::many0(Elf64_Sym::parse)(v).unwrap();
        Self { symbols: s }
    }
}

impl Into<Vec<u8>> for SymbolSection {
    fn into(self) -> Vec<u8> {
        let mut out: Vec<u8> = Vec::new();
        out.extend(
            self.symbols
                .iter()
                .map::<Vec<u8>, _>(|a| a.into())
                .flat_map(|f| f),
        );
        out
    }
}

impl Into<Vec<u8>> for &SymbolSection {
    fn into(self) -> Vec<u8> {
        let mut out: Vec<u8> = Vec::new();
        out.extend(
            self.symbols
                .iter()
                .map::<Vec<u8>, _>(|a| a.into())
                .flat_map(|f| f),
        );
        out
    }
}
