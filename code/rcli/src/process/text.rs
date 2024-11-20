use crate::{get_reader, TextSignFormat};
use anyhow::Result;
use base64::{
    engine::general_purpose::{STANDARD, URL_SAFE_NO_PAD},
    Engine as _,
};
use std::{fs, io::Read};

trait TextSign {
    /// Sign the data from the reader and return the signature
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>>; // 动态分发，运行效率低，编译体积小
}

trait TextVerity {
    fn verity(&self, reader: impl Read, sig: &[u8]) -> Result<bool>; // 静态分发，运行效率高，编译体积大
}

struct Blake3 {
    key: [u8; 32],
}

pub fn process_sign(input: &str, key: &str, format: TextSignFormat) -> Result<()> {
    let mut reader = get_reader(input)?;

    let signed = match format {
        TextSignFormat::Blake3 => {
            let key = fs::read(key)?;
            let key = &key[..32];
            let key = key.try_into()?;
            let signed = Blake3 { key };
            signed.sign(&mut reader)?
        }
        TextSignFormat::Ed25519 => todo!(),
    };
    let signed = URL_SAFE_NO_PAD.encode(&signed);
    println!("{}", signed);
    Ok(())
}

impl TextSign for Blake3 {
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>> {
        // TODO: improve perf by reading in chunks
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        Ok(blake3::keyed_hash(&self.key, &buf).as_bytes().to_vec())
    }
}

impl TextVerity for Blake3 {
    fn verity(&self, mut reader: impl Read, sig: &[u8]) -> Result<bool> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let hash = blake3::hash(&buf);
        let hash = hash.as_bytes();
        Ok(hash == sig)
    }
}
