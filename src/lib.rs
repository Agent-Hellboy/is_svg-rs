extern crate content_inspector;
extern crate regex;

use content_inspector::{inspect, ContentType};
use regex::Regex;
use std::fs;

pub fn is_svg(filename: &str) -> &str {
    let data = fs::read(&filename).expect("Unable to read file");
    if inspect(&data) == ContentType::BINARY {
        return "binary file please check imghdr to check this file";
    } else {
        let _re =
            Regex::new(r"(?:<\?xml\b[^>]*>[^<]*)?(?:<!--.*?-->[^<]*)*(?:<svg|<!DOCTYPE svg)\b")
                .unwrap();
        let joined = std::str::from_utf8(&data).unwrap();
        if _re.is_match(joined) {
            return "yes it is a svg file";
        } else {
            return "Not an svg file or corrupted file";
        }
    }
}
