use clap::Parser;
use rcli::{process_csv, Opts, SubCommand};

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    match opts.cmd {
        SubCommand::Csv(opts) => {
            let output = if let Some(output) = opts.output {
                output
            } else {
                format!("output.{}", Into::<&'static str>::into(opts.format))
            };
            process_csv(&opts.input, output, opts.format)?;
        }
    }
    Ok(())
}
