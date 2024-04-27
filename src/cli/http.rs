use super::verify_path;
use crate::{process_http_serve, CmdExecute};
use anyhow::Ok;
use clap::Parser;
use enum_dispatch::enum_dispatch;
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[enum_dispatch(CmdExecute)]
pub enum HttpSubCommand {
    #[command(about = "Serve a directory over http")]
    Serve(HttpServeOpts),
}

#[derive(Debug, Parser)]
pub struct HttpServeOpts {
    #[arg(short,long,value_parser=verify_path,default_value = ".")]
    pub dir: PathBuf,
    #[arg(short, long, default_value_t = 8080)]
    pub port: u16,
}

impl CmdExecute for HttpServeOpts {
    async fn execute(self) -> anyhow::Result<()> {
        process_http_serve(self.dir, self.port).await?;
        Ok(())
    }
}
