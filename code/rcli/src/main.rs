use std::fs;

use clap::Parser;
use rcli::{
    process_csv, process_decode, process_encode, process_genpass, process_text_generate,
    process_text_sign, process_text_verity, process_text_encrypt, process_text_decrypt,
};
use rcli::{Base64SubCommand, HttpSubCommand, Opts, SubCommand, TextSubCommand};
use zxcvbn::zxcvbn;

// rcli csv -i input.csv -o output.json --header -d ','

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let opts = Opts::parse();
    match opts.cmd {
        SubCommand::Csv(opts) => {
            let output = if let Some(output) = opts.output {
                output.clone()
            } else {
                format!("output.{}", opts.format)
            };
            process_csv(&opts.input, output, opts.format)?
        }
        SubCommand::GenPass(opts) => {
            let password = process_genpass(
                opts.length,
                opts.uppercase,
                opts.lowercase,
                opts.number,
                opts.symbol,
            )?;
            println!("{}", password);

            let estimate = zxcvbn(&password, &[]);
            eprintln!("Password strength: {}", estimate.score());
        }
        SubCommand::Base64(subcmd) => match subcmd {
            Base64SubCommand::Encode(opts) => {
                let encoded = process_encode(&opts.input, opts.format)?;
                println!("{}", encoded);
            }
            Base64SubCommand::Decode(opts) => {
                let decoded = process_decode(&opts.input, opts.format)?;
                // TODO: decoded data might not be string (but for this example, we assume it is)
                let decoded = String::from_utf8(decoded)?;
                println!("{}", decoded);
            }
        },
        SubCommand::Text(subcmd) => match subcmd {
            TextSubCommand::Sign(opts) => {
                let sig = process_text_sign(&opts.input, &opts.key, opts.format)?;
                println!("{}", sig);
            }
            TextSubCommand::Verity(opts) => {
                let verified = process_text_verity(&opts.input, &opts.key, opts.format, &opts.sig)?;
                println!("{}", verified);
            }
            TextSubCommand::Generate(opts) => {
                let key = process_text_generate(opts.format)?;
                match opts.format {
                    rcli::TextGenerateFormat::Blake3 => {
                        let name = opts.output.join("blake3.txt");
                        fs::write(name, &key[0])?;
                    }
                    rcli::TextGenerateFormat::Ed25519 => {
                        let name = &opts.output;
                        fs::write(name.join("ed25519.sk"), &key[0])?;
                        fs::write(name.join("ed25519.pk"), &key[1])?;
                    }
                    rcli::TextGenerateFormat::ChaCha20 => {
                        let name = &opts.output.join("chacha20.key");
                        fs::write(name, &key[0])?;
                    }
                    rcli::TextGenerateFormat::Aes => {
                        let name = &opts.output.join("aes_gcm.key");
                        fs::write(name, &key[0])?;
                    }
                }
            }
            TextSubCommand::Encrypt(opts) => {
                let data = process_text_encrypt(&opts.input, &opts.key, opts.format)?;
                println!("Encrypt data:{}", data)
            }
            TextSubCommand::Decrypt(opts) => {
                let data = process_text_decrypt(&opts.input, &opts.key, opts.format)?;
                println!("Decrypt data:{}", data)
            }
        },
        SubCommand::Http(subcmd) => match subcmd {
            HttpSubCommand::Serve(opts) => {
                println!("Serving at http://localhost:{}", opts.port)
            }
        },
    }

    Ok(())
}
