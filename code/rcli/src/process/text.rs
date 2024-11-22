use super::process_genpass;
use crate::{cli::TextEncryptFormat, cli::TextGenerateFormat, get_reader, TextSignFormat};
use anyhow::Result;
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use rand::rngs::OsRng;
use std::{fs, io::Read, path::Path};

use chacha20poly1305::{
    aead::{Aead, AeadCore, KeyInit},
    ChaCha20Poly1305,
};

use aes_gcm::{
    Aes256Gcm// Or `Aes128Gcm`
};

type GenerateKey = Vec<Vec<u8>>;

pub trait TextSign {
    /// Sign the data from the reader and return the signature
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>>; // 动态分发，运行效率低，编译体积小
}

pub trait TextVerity {
    /// Verity the data from the reader with the signature
    fn verity(&self, reader: impl Read, sig: &[u8]) -> Result<bool>; // 静态分发，运行效率高，编译体积大
}

pub trait TextEncrypt {
    fn encrypt(&self, reader: &mut dyn Read) -> Result<Vec<u8>>;
}

pub trait TextDecrypt {
    fn decrypt(&self, reader: &mut dyn Read) -> Result<Vec<u8>>;
}

trait KeyLoader {
    fn load(path: impl AsRef<Path>) -> Result<Self>
    where
        Self: Sized;
}

pub struct Blake3 {
    key: [u8; 32],
}

pub struct Ed25519Signer {
    key: SigningKey,
}

pub struct Ed25519Verifier {
    key: VerifyingKey,
}

pub struct ChaCha20 {
    key: chacha20poly1305::Key,
}

pub struct Aes {
    key: aes_gcm::Key<Aes256Gcm>
}
pub fn process_text_encrypt(input: &str, key: &str, format: TextEncryptFormat) -> Result<String> {
    let mut reader = get_reader(input)?;

    let encrypted = match format {
        TextEncryptFormat::ChaCha20 => {
            let cipher = ChaCha20::load(key)?;
            cipher.encrypt(&mut reader)?
        },
        TextEncryptFormat::Aes => {
            let cipher = Aes::load(key)?;
            cipher.encrypt(&mut reader)?
        }
    };
    let encrypted = URL_SAFE_NO_PAD.encode(&encrypted);
    Ok(encrypted)
}

pub fn process_text_decrypt(input: &str, key: &str, format: TextEncryptFormat) -> Result<String> {
    let mut reader = get_reader(input)?;
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;

    let encrypted = URL_SAFE_NO_PAD.decode(&buf)?;
    let decrypted = match format {
        TextEncryptFormat::ChaCha20 => {
            let cipher = ChaCha20::load(key)?;
            cipher.decrypt(&mut encrypted.as_slice())?
        },
        TextEncryptFormat::Aes => {
            let cipher = Aes::load(key)?;
            cipher.decrypt(&mut encrypted.as_slice())?
        }
    };
    let decrypted = String::from_utf8(decrypted)?;
    Ok(decrypted)
}

pub fn process_text_sign(input: &str, key: &str, format: TextSignFormat) -> Result<String> {
    let mut reader = get_reader(input)?;

    let signed = match format {
        TextSignFormat::Blake3 => {
            let signer = Blake3::load(key)?;
            signer.sign(&mut reader)?
        }
        TextSignFormat::Ed25519 => {
            let signer = Ed25519Signer::load(key)?;
            signer.sign(&mut reader)?
        }
    };
    let signed = URL_SAFE_NO_PAD.encode(&signed);
    Ok(signed)
}

pub fn process_text_verity(
    input: &str,
    key: &str,
    format: TextSignFormat,
    sig: &str,
) -> Result<bool> {
    let reader = get_reader(input)?;

    let sig = URL_SAFE_NO_PAD.decode(sig)?;
    let verified = match format {
        TextSignFormat::Blake3 => {
            let verifier = Blake3::load(key)?;
            verifier.verity(reader, &sig)?
        }
        TextSignFormat::Ed25519 => {
            let verifier = Ed25519Verifier::load(key)?;
            verifier.verity(reader, &sig)?
        }
    };

    Ok(verified)
}

pub fn process_text_generate(format: TextGenerateFormat) -> Result<GenerateKey> {
    let keys = match format {
        TextGenerateFormat::Blake3 => Blake3::generate()?,
        TextGenerateFormat::Ed25519 => Ed25519Signer::generate()?,
        TextGenerateFormat::ChaCha20 => ChaCha20::generate()?,
        TextGenerateFormat::Aes => Aes::generate()?,
    };

    Ok(keys)
}

impl TextSign for Blake3 {
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>> {
        // TODO: improve perf by reading in chunks
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        Ok(blake3::keyed_hash(&self.key, &buf).as_bytes().to_vec())
    }
}

impl TextSign for Ed25519Signer {
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let sig = self.key.sign(&buf);
        Ok(sig.to_bytes().to_vec())
    }
}

impl TextVerity for Blake3 {
    fn verity(&self, mut reader: impl Read, sig: &[u8]) -> Result<bool> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let hash = blake3::keyed_hash(&self.key, &buf);
        let hash = hash.as_bytes();
        Ok(hash == sig)
    }
}

impl TextVerity for Ed25519Verifier {
    fn verity(&self, mut reader: impl Read, sig: &[u8]) -> Result<bool> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let sig = Signature::from_bytes(sig.try_into()?);
        let ret = self.key.verify(&buf, &sig).is_ok();
        Ok(ret)
    }
}

impl TextEncrypt for ChaCha20 {
    fn encrypt(&self, reader: &mut dyn Read) -> Result<Vec<u8>> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;

        let cipher: ChaCha20Poly1305 = ChaCha20Poly1305::new(&self.key);

        let nonce: chacha20poly1305::Nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng);
        let encrypted_data: Vec<u8> = cipher
            .encrypt(&nonce, buf.as_ref())
            .expect("Encryption failure");

        let mut rnt = nonce.to_vec();
        rnt.extend(encrypted_data.iter());
        Ok(rnt)
    }
}

impl TextDecrypt for ChaCha20 {
    fn decrypt(&self, reader: &mut dyn Read) -> Result<Vec<u8>> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;

        let cipher: ChaCha20Poly1305 = ChaCha20Poly1305::new(&self.key);

        let nonce: &chacha20poly1305::Nonce = chacha20poly1305::Nonce::from_slice(&buf[..12]);
        let encrypted_data = &buf[12..];

        let decrypted_data: Vec<u8> = cipher
            .decrypt(nonce, encrypted_data)
            .expect("Encryption failure");

        Ok(decrypted_data)
    }
}

impl TextEncrypt for Aes {
    fn encrypt(&self, reader: &mut dyn Read) -> Result<Vec<u8>> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;

        let cipher: Aes256Gcm  = Aes256Gcm::new(&self.key);

        let nonce  = Aes256Gcm::generate_nonce(&mut OsRng);
        let encrypted_data: Vec<u8> = cipher
            .encrypt(&nonce, buf.as_ref())
            .expect("Encryption failure");

        let mut rnt = nonce.to_vec();
        rnt.extend(encrypted_data.iter());
        Ok(rnt)
    }
}

impl TextDecrypt for Aes {
    fn decrypt(&self, reader: &mut dyn Read) -> Result<Vec<u8>> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;

        let cipher: Aes256Gcm = Aes256Gcm::new(&self.key);

        let nonce  = aes_gcm::Nonce::from_slice(&buf[..12]);
        let encrypted_data = &buf[12..];

        let decrypted_data: Vec<u8> = cipher
            .decrypt(nonce, encrypted_data)
            .expect("Encryption failure");

        Ok(decrypted_data)
    }
}

impl Blake3 {
    pub fn new(key: [u8; 32]) -> Self {
        Blake3 { key }
    }

    pub fn try_new(key: &[u8]) -> Result<Self> {
        let key = &key[..32];
        let key = key.try_into()?;
        let signer = Blake3::new(key);
        Ok(signer)
    }
}

impl Ed25519Signer {
    pub fn new(key: SigningKey) -> Self {
        Self { key }
    }

    pub fn try_new(key: &[u8]) -> Result<Self> {
        let key = SigningKey::from_bytes(key.try_into()?);
        let signer = Self::new(key);
        Ok(signer)
    }
}

impl Ed25519Verifier {
    pub fn new(key: VerifyingKey) -> Self {
        Self { key }
    }

    pub fn try_new(key: &[u8]) -> Result<Self> {
        let key = VerifyingKey::from_bytes(key.try_into()?)?;
        let signer = Self::new(key);
        Ok(signer)
    }
}

impl ChaCha20 {
    pub fn new(key: chacha20poly1305::Key) -> Self {
        Self { key }
    }

    pub fn try_new(key: &[u8]) -> Result<Self> {
        let key: chacha20poly1305::Key = chacha20poly1305::Key::clone_from_slice(key);
        Ok(Self::new(key))
    }
}

impl Aes {
    pub fn new(key: aes_gcm::Key<Aes256Gcm>) -> Self {
        Self { key }
    }

    pub fn try_new(key: &[u8]) -> Result<Self> {
        let key: &aes_gcm::Key<Aes256Gcm> = aes_gcm::Key::<Aes256Gcm>::from_slice(key);

        Ok(Self::new(*key))
    }
}

impl KeyLoader for Aes {
    fn load(path: impl AsRef<Path>) -> Result<Self>
        where
            Self: Sized {
        let key = fs::read(path)?;
        Self::try_new(&key)
    }
}

impl KeyLoader for Blake3 {
    fn load(path: impl AsRef<Path>) -> Result<Self>
    where
        Self: Sized,
    {
        let key = fs::read(path)?;
        Self::try_new(&key)
    }
}

impl KeyLoader for Ed25519Signer {
    fn load(path: impl AsRef<Path>) -> Result<Self>
    where
        Self: Sized,
    {
        let key = fs::read(path)?;
        Self::try_new(&key)
    }
}

impl KeyLoader for Ed25519Verifier {
    fn load(path: impl AsRef<Path>) -> Result<Self>
    where
        Self: Sized,
    {
        let key = fs::read(path)?;
        Self::try_new(&key)
    }
}

impl KeyLoader for ChaCha20 {
    fn load(path: impl AsRef<Path>) -> Result<Self>
    where
        Self: Sized,
    {
        let key = fs::read(path)?;
        Self::try_new(&key)
    }
}

pub trait KeyGenerator {
    fn generate() -> Result<GenerateKey>;
}

impl KeyGenerator for Blake3 {
    fn generate() -> Result<GenerateKey> {
        let key = process_genpass(32, true, true, true, true)?;
        let key = key.as_bytes().to_vec();
        Ok(vec![key])
    }
}

impl KeyGenerator for Ed25519Signer {
    fn generate() -> Result<GenerateKey> {
        let mut csprng = OsRng;
        let sk: SigningKey = SigningKey::generate(&mut csprng);
        let pk: Vec<u8> = sk.verifying_key().as_bytes().to_vec();
        let sk: Vec<u8> = sk.as_bytes().to_vec();

        Ok(vec![sk, pk])
    }
}

impl KeyGenerator for ChaCha20 {
    fn generate() -> Result<GenerateKey> {
        let mut csprng = OsRng;
        let key: chacha20poly1305::Key = ChaCha20Poly1305::generate_key(&mut csprng);
        let key: Vec<u8> = key.to_vec();
        Ok(vec![key])
    }
}

impl KeyGenerator for Aes {
    fn generate() -> Result<GenerateKey> {
        let mut csprng = OsRng;
        let key = Aes256Gcm::generate_key(&mut csprng);
        let key: Vec<u8> = key.to_vec();
        Ok(vec![key])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blake3_sign_verify() -> Result<()> {
        let blake3 = Blake3::load("fixtures/blake3.txt")?;

        let data = b"hello world";
        let sig = blake3.sign(&mut &data[..]).unwrap();

        assert!(blake3.verity(&data[..], &sig).unwrap());
        Ok(())
    }

    #[test]
    fn test_ed25519_sign_verify() -> Result<()> {
        let sk = Ed25519Signer::load("fixtures/ed25519.sk")?;
        let pk = Ed25519Verifier::load("fixtures/ed25519.pk")?;

        let data = b"hello world";
        let sig = sk.sign(&mut &data[..]).unwrap();
        assert!(pk.verity(&data[..], &sig).unwrap());
        Ok(())
    }

    #[test]
    fn test_chacha20_generate_load() -> Result<()> {
        let key = ChaCha20::generate()?;
        fs::write("fixtures/chacha20.key", &key[0])?;
        let read_key = ChaCha20::load("fixtures/chacha20.key")?;

        assert_eq!(key[0], read_key.key.to_vec());
        Ok(())
    }

    #[test]
    fn test_chacha20_encrypt_decrypt() -> Result<()> {
        let chacha20 = ChaCha20::load("fixtures/chacha20.key")?;

        let data = b"hello world";
        let encrypted = chacha20.encrypt(&mut &data[..])?;
        let decrypted = chacha20.decrypt(&mut encrypted.as_slice())?;

        assert_eq!(decrypted.as_slice(), data);
        Ok(())
    }
}
