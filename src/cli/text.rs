use super::{verify_file, verify_path};
use crate::{
    process_decrypt, process_encrypt, process_sign, process_verify, text::process_generate_key,
    CmdExecute,
};
use clap::Parser;
use enum_dispatch::enum_dispatch;
use std::{path::PathBuf, str::FromStr};

#[derive(Debug, Parser)]
#[enum_dispatch(CmdExecute)]
pub enum TextSubCommand {
    #[command(about = "Sign a message")]
    Sign(TextSignOpts),
    #[command(about = "Verify a message")]
    Verify(TextVerifyOpts),
    #[command(about = "Generate a new key")]
    GenerateKey(TextKeyGenerateOpts),
    #[command(about = "Encrypt a message with chacha20poly1305")]
    Encrypt(EncryptOpts),
    #[command(about = "Decrypt a message with chacha20poly1305")]
    Decrypt(DecryptOpts),
}

#[derive(Debug, Parser)]
pub struct TextSignOpts {
    #[arg(short,long,value_parser=verify_file,default_value = "-")]
    pub input: String,
    #[arg(short,long,value_parser=verify_file)]
    pub key: String,
    #[arg(long,default_value="blake3",value_parser=parse_format)]
    pub format: TextSignFormat,
}

#[derive(Debug, Parser)]
pub struct TextVerifyOpts {
    #[arg(short,long,value_parser=verify_file,default_value = "-")]
    pub input: String,
    #[arg(short, long,value_parser=verify_file)]
    pub key: String,
    #[arg(short, long)]
    pub sig: String,
    #[arg(long,default_value="blake3",value_parser=parse_format)]
    pub format: TextSignFormat,
}

#[derive(Debug, Parser)]
pub struct TextKeyGenerateOpts {
    #[arg(long,default_value="blake3",value_parser=parse_format)]
    pub format: TextSignFormat,
    #[arg(short,long,value_parser=verify_path)]
    pub output: PathBuf,
}

#[derive(Debug, Parser)]
pub struct EncryptOpts {
    #[arg(short,long,value_parser=verify_file,default_value = "-")]
    pub input: String,
    #[arg(short, long)]
    pub key: String,
    #[arg(short, long)]
    pub nonce: String,
}

#[derive(Debug, Parser)]
pub struct DecryptOpts {
    #[arg(short,long,value_parser=verify_file,default_value = "-")]
    pub input: String,
    #[arg(short, long)]
    pub key: String,
    #[arg(short, long)]
    pub nonce: String,
}

#[derive(Debug, Clone, Copy)]
pub enum TextSignFormat {
    Blake3,
    Ed25519,
}

fn parse_format(format: &str) -> Result<TextSignFormat, anyhow::Error> {
    format.parse()
}

impl FromStr for TextSignFormat {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "blake3" => Ok(TextSignFormat::Blake3),
            "ed25519" => Ok(TextSignFormat::Ed25519),
            _ => Err(anyhow::anyhow!("Invalid format")),
        }
    }
}
impl From<TextSignFormat> for &'static str {
    fn from(format: TextSignFormat) -> Self {
        match format {
            TextSignFormat::Blake3 => "blake3",
            TextSignFormat::Ed25519 => "ed25519",
        }
    }
}

impl std::fmt::Display for TextSignFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}

impl CmdExecute for TextSignOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let sig = process_sign(&self.input, &self.key, self.format)?;
        println!("Signature: {}", sig);
        Ok(())
    }
}

impl CmdExecute for TextVerifyOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let res = process_verify(&self.input, &self.key, self.sig.clone(), self.format)?;
        println!("verify result is {}", res);
        Ok(())
    }
}

impl CmdExecute for TextKeyGenerateOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let kyes = process_generate_key(self.format)?;
        match self.format {
            TextSignFormat::Blake3 => {
                let name = self.output.join("blake3.txt");
                tokio::fs::write(name, &kyes[0]).await?;
            }
            TextSignFormat::Ed25519 => {
                let name = self.output;
                tokio::fs::write(name.join("ed25519.sk"), &kyes[0]).await?;
                let name = name.join("ed25519.pk");
                tokio::fs::write(name, &kyes[1]).await?;
            }
        }
        Ok(())
    }
}

impl CmdExecute for EncryptOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let res = process_encrypt(&self.input, self.key, self.nonce)?;
        println!("encrypt result is {}", res);
        Ok(())
    }
}

impl CmdExecute for DecryptOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let res = process_decrypt(&self.input, self.key, self.nonce)?;
        println!("encrypt result is {}", res);
        Ok(())
    }
}
