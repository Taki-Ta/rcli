use super::verify_file;
use crate::{process_jwt_sign, process_jwt_verify, CmdExecute};
use anyhow::Ok;
use clap::Parser;
use enum_dispatch::enum_dispatch;
use parse_duration::parse as duration_parser;
use std::time::Duration;

#[derive(Debug, Parser)]
#[enum_dispatch(CmdExecute)]
pub enum JWTSubCommand {
    #[command(about = "Sign a message as jwt")]
    Sign(JWTSignOpts),
    #[command(about = "Verify jst signature")]
    Verify(JWTVerifyOpts),
}

#[derive(Debug, Parser)]
pub struct JWTSignOpts {
    #[arg(short,long,value_parser=verify_file,default_value = "-")]
    pub key: String,
    #[arg(short, long)]
    pub sub: Option<String>,
    #[arg(short, long)]
    pub aud: Option<String>,
    #[arg(short,long,value_parser=duration_parser)]
    pub exp: Option<Duration>,
}

#[derive(Debug, Parser)]
pub struct JWTVerifyOpts {
    #[arg(short,long,value_parser=verify_file,default_value = "-")]
    pub input: String,
    #[arg(short, long)]
    pub token: String,
    //TODO add validation parameter
    // #[arg(short, long)]
    // validation:Validation
}

impl CmdExecute for JWTSignOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let jwt = process_jwt_sign(&self.key, self.sub, self.aud, self.exp)?;
        println!("{}", jwt);
        Ok(())
    }
}

impl CmdExecute for JWTVerifyOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let res = process_jwt_verify(&self.input, &self.token)?;
        println!("verify result is {}", res);
        Ok(())
    }
}
