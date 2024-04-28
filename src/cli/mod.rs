mod base64;
mod csv;
mod genpass;
mod http;
mod jwt;
mod text;

pub use self::{base64::*, csv::*, genpass::*, http::*, jwt::*, text::*};
use clap::Parser;
use enum_dispatch::enum_dispatch;
use std::path::{Path, PathBuf};

#[derive(Debug, Parser)]
#[command(name="rcli",version,author,about,long_about=None )]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Parser)]
#[enum_dispatch(CmdExecute)]
pub enum SubCommand {
    #[command(name = "csv", about = "convert csv to other formats")]
    Csv(CsvOpts),
    #[command(name = "genpass", about = "generate password")]
    Genpass(GenPassOpts),
    #[command(subcommand, about = "base64 encode/decode")]
    Base64(Bass64SubCommand),
    #[command(subcommand, about = "text sign/verify")]
    Text(TextSubCommand),
    #[command(subcommand, about = "http serve")]
    Http(HttpSubCommand),
    #[command(subcommand, about = "jwt sign/verify")]
    JWT(JWTSubCommand),
}

fn verify_file(filename: &str) -> Result<String, &'static str> {
    //check if file exists
    if filename == "-" || Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("File does not exist")
    }
}

fn verify_path(path: &str) -> Result<PathBuf, &'static str> {
    //check if file exists
    if Path::new(path).exists() && Path::new(path).is_dir() {
        Ok(path.into())
    } else {
        Err("File does not exist or is not a directory")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_verify_input_file() {
        assert_eq!(verify_file("Cargo.toml"), Ok("Cargo.toml".into()));
        assert_eq!(verify_file("non-existent-file"), Err("File does not exist"));
        assert_eq!(verify_file("-"), Ok("-".into()));
        assert_eq!(verify_file("*"), Err("File does not exist"));
    }
}
