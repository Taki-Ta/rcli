use clap::Parser;
use rcli::{
    csv_convert::process_csv,
    gen_pass::process_genpass,
    process_decode, process_encode,
    text::{process_generate_key, process_sign, process_verify},
    Bass64SubCommand, Opts, SubCommand, TextSignFormat, TextSubCommand,
};
use zxcvbn::zxcvbn;

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
            let pass = process_genpass(
                opts.length,
                opts.uppercase,
                opts.lowercase,
                opts.number,
                opts.symbols,
            )?;
            println!("Password: {}", &pass);
            let estimate = zxcvbn(&pass, &[]).unwrap();
            println!("password strength is {:?}", estimate.score());
        }
        SubCommand::Base64(subcmd) => match subcmd {
            Bass64SubCommand::Encode(opts) => {
                let encode = process_encode(&opts.input, opts.format)?;
                println!("encode : {}", encode);
            }
            Bass64SubCommand::Decode(opts) => {
                let decode = process_decode(&opts.input, opts.format)?;
                let decode = String::from_utf8(decode)?;
                println!("decode : {}", decode);
            }
        },
        SubCommand::Text(subcmd) => match subcmd {
            TextSubCommand::Sign(opts) => {
                let sig = process_sign(&opts.input, &opts.key, opts.format)?;
                println!("Signature: {}", sig);
            }
            TextSubCommand::Verify(opts) => {
                let res = process_verify(&opts.input, &opts.key, opts.sig.clone(), opts.format)?;
                println!("verify result is {}", res);
            }
            TextSubCommand::GenerateKey(opts) => {
                let kyes = process_generate_key(opts.format)?;
                match opts.format {
                    TextSignFormat::Blake3 => {
                        let name = opts.output.join("blake3.txt");
                        std::fs::write(name, &kyes[0])?;
                    }
                    TextSignFormat::Ed25519 => {
                        let name = opts.output;
                        std::fs::write(name.join("ed25519.sk"), &kyes[0])?;
                        let name = name.join("ed25519.pk");
                        std::fs::write(name, &kyes[1])?;
                    }
                }
            }
        },
    }
    Ok(())
}
