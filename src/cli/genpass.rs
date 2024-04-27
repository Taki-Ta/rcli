use crate::{process_genpass, CmdExecute};
use anyhow::Ok;
use clap::Parser;
use zxcvbn::zxcvbn;

#[derive(Debug, Parser)]
pub struct GenPassOpts {
    #[arg(short, long, default_value_t = 16)]
    pub length: u8,
    #[arg(long, default_value_t = true)]
    pub number: bool,
    #[arg(long, default_value_t = true)]
    pub symbols: bool,
    #[arg(long, default_value_t = true)]
    pub uppercase: bool,
    #[arg(long, default_value_t = true)]
    pub lowercase: bool,
}

impl CmdExecute for GenPassOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let pass = process_genpass(
            self.length,
            self.uppercase,
            self.lowercase,
            self.number,
            self.symbols,
        )?;
        println!("Password: {}", &pass);
        let estimate = zxcvbn(&pass, &[]).unwrap();
        println!("password strength is {:?}", estimate.score());
        Ok(())
    }
}
