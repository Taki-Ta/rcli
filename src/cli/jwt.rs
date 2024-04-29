use crate::{process_jwt_sign, process_jwt_verify, CmdExecute};
use anyhow::Ok;
use clap::Parser;
use enum_dispatch::enum_dispatch;
use jsonwebtoken::Algorithm;
use parse_duration::parse;

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
    #[arg(short, long)]
    pub sub: String,
    #[arg(short, long)]
    pub aud: String,
    #[arg(short,long,value_parser=duration_parser)]
    pub exp: usize,
    #[arg(long, default_value = "HS256")]
    pub alg: Algorithm,
}

#[derive(Debug, Parser)]
pub struct JWTVerifyOpts {
    #[arg(short, long)]
    pub token: String,
    #[arg(long, default_value = "HS256")]
    pub alg: Algorithm,
    #[arg(long)]
    pub aud: String,
}

fn duration_parser(input: &str) -> anyhow::Result<usize> {
    let duration = parse(input)?;
    Ok(duration.as_secs() as usize)
}

impl CmdExecute for JWTSignOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let token = process_jwt_sign(self.sub, self.exp, self.aud, self.alg)?;
        println!("{}", token);
        Ok(())
    }
}

impl CmdExecute for JWTVerifyOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let res = process_jwt_verify(self.token, self.aud, self.alg)?;
        println!("{}", res);
        Ok(())
    }
}
