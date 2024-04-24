mod base64;
mod csv;
mod genpass;

pub use self::base64::{Base64Format, Bass64SubCommand};
pub use self::csv::OutputFormat;
use self::{csv::CsvOpts, genpass::GenPassOpts};
use clap::Parser;
use std::path::Path;

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
}

fn verify_input_file(filename: &str) -> Result<String, &'static str> {
    //check if file exists
    if filename == "-" || Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("File does not exist")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_verify_input_file() {
        assert_eq!(verify_input_file("Cargo.toml"), Ok("Cargo.toml".into()));
        assert_eq!(
            verify_input_file("non-existent-file"),
            Err("File does not exist")
        );
        assert_eq!(verify_input_file("-"), Ok("-".into()));
        assert_eq!(verify_input_file("*"), Err("File does not exist"));
    }
}
