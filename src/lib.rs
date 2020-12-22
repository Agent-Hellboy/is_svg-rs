extern crate content_inspector;
extern crate regex;

use content_inspector::{inspect, ContentType};
use regex::Regex;
use std::fs;
use std::io;
use std::string::FromUtf8Error;
use lazy_static::lazy_static;

lazy_static! { // using lazy_static because compiling regex is expensive
    static ref SVG_REGEX: Regex = Regex::new(r"(?:<\?xml\b[^>]*>[^<]*)?(?:<!--.*?-->[^<]*)*(?:<svg|<!DOCTYPE svg)\b")
    .unwrap(); // this is OK because we know that the regex compiles
}

pub fn is_svg(filename: &str) -> Result<bool, Error> {
    let data = fs::read(&filename)?;

    let content_type = inspect(&data);
    if (content_type != ContentType::UTF_8) && (content_type != ContentType::UTF_8_BOM) {
        unimplemented!("currently doesn't support UTF-8")
    } else {
        let joined = String::from_utf8(data)?;
        Ok(SVG_REGEX.is_match(&joined))
    }
}

#[derive(Debug)]
pub enum Error { // so that we can return our own appropriate error
    IOError(io::Error),
    Utf8ParseError(FromUtf8Error),
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(match self {
            Error::IOError(e) => e,
            Error::Utf8ParseError(e) => e
        })
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::IOError(e) => write!(f, "couldn't read from file: {}", e),
            Error::Utf8ParseError(e) => write!(f, "couldn't parse utf-8: {}", e)
        }
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::IOError(e)
    }
}

impl From<FromUtf8Error> for Error {
    fn from(e: FromUtf8Error) -> Self {
        Error::Utf8ParseError(e)
    }
}