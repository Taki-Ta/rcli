mod base64;
mod csv;
mod genpass;
mod text;

pub use self::base64::{Base64Format, Bass64SubCommand};
pub use self::csv::OutputFormat;
use self::{csv::CsvOpts, genpass::GenPassOpts};
pub use self::{text::TextSignFormat, text::TextSubCommand};
use clap::Parser;
use std::path::{Path, PathBuf};

#[derive(Debug, Parser)]
#[command(name="rcli",version,author,about,long_about=None )]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Parser)]
pub enum SubCommand {
    #[command(name = "csv", about = "convert csv to other formats")]
    Csv(CsvOpts),
    #[command(name = "genpass", about = "generate password")]
    Genpass(GenPassOpts),
    #[command(subcommand)]
    Base64(Bass64SubCommand),
    #[command(subcommand)]
    Text(TextSubCommand),
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
