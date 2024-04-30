use std::io::Error as IOError;

use strong_xml::XmlError;
use zip::result::ZipError;

use thiserror::Error;

/// Error type of docx-rs
#[derive(Error, Debug)]
pub enum DocxError {
    #[error("io error")]
    IO(#[from] IOError),
    #[error("xml error")]
    Xml(#[from] XmlError),
    #[error("zip error")]
    Zip(#[from] ZipError),
}

/// Specialized `Result` which the error value is `DocxError`.
pub type DocxResult<T> = Result<T, DocxError>;
