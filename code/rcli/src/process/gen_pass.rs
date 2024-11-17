use anyhow::Ok;
use rand::Rng;

use crate::opts::GenPassOpts;

pub fn process_genpass(opts: &GenPassOpts) -> anyhow::Result<()> {
    let mut rng = rand::thread_rng();
    let mut password = String::new();
    let mut chars = Vec::new();

    if opts.uppercase {
        chars.extend_from_slice(b"ABCDEFGHJKMNPQRSTUVWSYZ");
    }

    if opts.lowercase {
        chars.extend_from_slice(b"abcdefghjkmnpqrstuvwsyz");
    }

    if opts.number {
        chars.extend_from_slice(b"123456789");
    }

    if opts.symbol {
        chars.extend_from_slice(b"!@#$%^&*_");
    }

    for _ in 0..opts.length {
        let idx = rng.gen_range(0..chars.len());
        password.push(chars[idx] as char);
    }

    Ok(())
}