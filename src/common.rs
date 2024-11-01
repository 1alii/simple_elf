use nom::error::Error;
use nom::error::ParseError;
use nom::IResult;
use serde::Serialize;
use std::fmt::Debug;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ParserIn<'a> {
    pub whole_file: &'a [u8],
    pub remaining: &'a [u8],
}

pub type ParserOut<'a, T> = IResult<ParserIn<'a>, T, Error<&'a [u8]>>;

impl<'a> From<(&'a [u8], &'a [u8])> for ParserIn<'a> {
    fn from(value: (&'a [u8], &'a [u8])) -> Self {
        Self {
            whole_file: value.0,
            remaining: value.1,
        }
    }
}

impl<'a> ParseError<ParserIn<'a>> for Error<&[u8]> {
    /*
     * not a real implementation
     * just to make the compiler stop screaming
     */
    fn append(_input: ParserIn<'a>, _kind: nom::error::ErrorKind, other: Self) -> Self {
        other
    }
    fn from_error_kind(_input: ParserIn<'a>, kind: nom::error::ErrorKind) -> Self {
        Self::new(&[1], kind)
    }
}

#[derive(Default)]
pub struct RawBinaryData {
    pub inner: Vec<u8>,
}

impl Serialize for RawBinaryData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(format!("data with length {}", self.inner.len()).as_str())
    }
}
// impl Default for RawBinaryData {
//     fn default() -> Self {
//         Self { inner: Vec::new() }
//     }
// }

impl RawBinaryData {
    pub fn len(&self) -> usize {
        self.inner.len()
    }
    pub fn push(&mut self, value: u8) {
        self.inner.push(value);
    }
    pub fn extend<T: IntoIterator<Item = u8>>(&mut self, value: T) {
        self.inner.extend(value);
    }
}

impl<T: AsRef<[u8]>> From<T> for RawBinaryData {
    fn from(value: T) -> Self {
        let tmp = value.as_ref();
        Self {
            inner: Vec::from(tmp),
        }
    }
}

impl Into<Vec<u8>> for RawBinaryData {
    fn into(self) -> Vec<u8> {
        self.inner
    }
}

impl Into<Vec<u8>> for &RawBinaryData {
    fn into(self) -> Vec<u8> {
        self.inner.clone()
    }
}

impl Debug for RawBinaryData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "data with length: {}", self.inner.len())?;
        Ok(())
    }
}
