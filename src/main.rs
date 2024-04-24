use clap::Parser;
use rcli::{
    csv_convert::process_csv, gen_pass::process_genpass, precess_decode, precess_encode,
    Bass64SubCommand, Opts, SubCommand,
};

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
        SubCommand::Genpass(opts) => {
            process_genpass(
                opts.length,
                opts.uppercase,
                opts.lowercase,
                opts.number,
                opts.symbols,
            )?;
        }
        SubCommand::Base64(subcmd) => match subcmd {
            Bass64SubCommand::Encode(opts) => {
                precess_encode(&opts.input, opts.format)?;
            }
            Bass64SubCommand::Decode(opts) => {
                precess_decode(&opts.input, opts.format)?;
            }
        },
    }
    Ok(())
}
