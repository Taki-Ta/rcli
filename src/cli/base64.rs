use super::verify_file;
use crate::{process_decode, process_encode, CmdExecute};
use clap::Parser;
use enum_dispatch::enum_dispatch;
use std::str::FromStr;

#[derive(Debug, Parser)]
#[enum_dispatch(CmdExecute)]
pub enum Bass64SubCommand {
    #[command(name = "encode", about = "encode base64")]
    Encode(Base64EncodeOpts),
    #[command(name = "decode", about = "decode base64")]
    Decode(Base64DecodeOpts),
}

#[derive(Debug, Parser)]
pub struct Base64EncodeOpts {
    #[arg(short,long,value_parser=verify_file,default_value = "-")]
    pub input: String,
    #[arg(short,long,value_parser=parse_base64_format,default_value = "standard")]
    pub format: Base64Format,
}

#[derive(Debug, Parser)]
pub struct Base64DecodeOpts {
    #[arg(short,long,value_parser=verify_file,default_value = "-")]
    pub input: String,
    #[arg(short,long,value_parser=parse_base64_format,default_value = "standard")]
    pub format: Base64Format,
}

#[derive(Debug, Clone, Copy)]
pub enum Base64Format {
    Standard,
    UrlSafe,
}

impl FromStr for Base64Format {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "standard" => Ok(Base64Format::Standard),
            "urlsafe" => Ok(Base64Format::UrlSafe),
            _ => Err(anyhow::anyhow!("Invalid format")),
        }
    }
}

fn parse_base64_format(format: &str) -> Result<Base64Format, anyhow::Error> {
    format.parse()
}

impl From<Base64Format> for &'static str {
    fn from(format: Base64Format) -> Self {
        match format {
            Base64Format::Standard => "standard",
            Base64Format::UrlSafe => "urlsafe",
        }
    }
}

impl std::fmt::Display for Base64Format {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}

impl CmdExecute for Base64EncodeOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let encode = process_encode(&self.input, self.format)?;
        println!("encode : {}", encode);
        Ok(())
    }
}

impl CmdExecute for Base64DecodeOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let decode = process_decode(&self.input, self.format)?;
        let decode = String::from_utf8(decode)?;
        println!("decode : {}", decode);
        Ok(())
    }
}
