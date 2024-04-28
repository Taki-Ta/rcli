use crate::cli::Base64Format;
use crate::utils::get_reader;
use anyhow::{Ok, Result};
use base64::{
    engine::general_purpose::{STANDARD, URL_SAFE_NO_PAD},
    Engine as _,
};
use std::io::Read;

pub fn process_encode(input: &str, format: Base64Format) -> Result<String> {
    //set if and else condition to return same type value
    let mut reader: Box<dyn Read> = get_reader(input)?;
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer)?;

    let encode = match format {
        Base64Format::Standard => STANDARD.encode(&buffer),
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.encode(&buffer),
    };
    Ok(encode)
}

pub fn process_decode(input: &str, format: Base64Format) -> Result<Vec<u8>> {
    let mut reader: Box<dyn Read> = get_reader(input)?;
    let mut buffer = String::new();
    reader.read_to_string(&mut buffer)?;
    let buffer = buffer.trim_end();
    let decode = match format {
        Base64Format::Standard => STANDARD.decode(buffer)?,
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.decode(buffer)?,
    };
    //TODO decode data might not be string
    Ok(decode)
}

#[cfg(test)]
mod tests {
    // #[test]
    // fn test_process_encode(){
    //     use super::*;
    //     assert_eq!(precess_encode("Cargo.toml", Base64Format::Standard), Ok(()));
    //     assert_eq!(precess_encode("Cargo.toml", Base64Format::UrlSafe), Ok(()));
    // }

    // #[test]
    // fn test_process_decode() {
    //     use super::*;
    //     let input = "fixtures/b64.txt";
    //     let format = Base64Format::UrlSafe;
    //     process_decode(input, format).unwrap();
    // }
}
