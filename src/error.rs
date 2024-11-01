use std::path::PathBuf;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, PartialEq)]
pub enum Error {
    HunspellLibError(i32),
    NegativeListLength(i32),
    NullPtr,
    AffixFileIsNoFile(String),
    DictionaryFileIsNoFile(String),
    CannotAddMoreDictionaries(PathBuf),
    Utf8Error(core::str::Utf8Error),
    NulError(std::ffi::NulError),
}

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(fmt, "{self:?}")
    }
}

impl From<core::str::Utf8Error> for Error {
    fn from(value: core::str::Utf8Error) -> Self {
        Self::Utf8Error(value)
    }
}

impl From<std::ffi::NulError> for Error {
    fn from(value: std::ffi::NulError) -> Self {
        Self::NulError(value)
    }
}

impl core::error::Error for Error {}
