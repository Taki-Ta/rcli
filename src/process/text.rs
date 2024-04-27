use crate::utils::get_reader;
use crate::{cli::TextSignFormat, gen_pass};
use anyhow::{Ok, Result};
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use ed25519::Signature;
use ed25519_dalek::{Signer, SigningKey, Verifier, VerifyingKey};
use rand::rngs::OsRng;
use std::{fs, io::Read, path::Path};

pub trait TextSign {
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>>;
}

pub trait TextVerify {
    fn verify(&self, reader: &mut dyn Read, sig: &[u8]) -> Result<bool>;
}

pub struct Blake3 {
    key: [u8; 32],
}

pub struct Ed25519Sign {
    key: SigningKey,
}

struct Ed25519Verify {
    key: VerifyingKey,
}

pub trait KeyLoader {
    fn load(key: impl AsRef<Path>) -> Result<Self>
    where
        Self: Sized;
}

pub trait KeyGenerator {
    fn generate() -> Result<Vec<Vec<u8>>>;
}

impl TextSign for Blake3 {
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>> {
        let mut buffer = Vec::new();
        reader.read_to_end(&mut buffer)?;
        Ok(blake3::keyed_hash(&self.key, &buffer).as_bytes().to_vec())
    }
}

impl TextVerify for Blake3 {
    fn verify(&self, reader: &mut dyn Read, sig: &[u8]) -> Result<bool> {
        let mut buffer = Vec::new();
        reader.read_to_end(&mut buffer)?;
        let hash = blake3::keyed_hash(&self.key, &buffer);
        let hash = hash.as_bytes();
        Ok(hash == sig)
    }
}

impl KeyLoader for Blake3 {
    fn load(key: impl AsRef<Path>) -> Result<Self> {
        let key = fs::read(key)?;
        let key: [u8; 32] = key[..32].try_into()?;
        Ok(Self { key })
    }
}

impl TextSign for Ed25519Sign {
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>> {
        let mut buffer = Vec::new();
        reader.read_to_end(&mut buffer)?;
        let sig = self.key.sign(&buffer);
        Ok(sig.to_bytes().to_vec())
    }
}

impl KeyLoader for Ed25519Sign {
    fn load(key: impl AsRef<Path>) -> Result<Self> {
        let key = fs::read(key)?;
        let key: [u8; 32] = key[..32].try_into()?;
        let key = SigningKey::from_bytes(&key);
        Ok(Self { key })
    }
}

impl TextVerify for Ed25519Verify {
    fn verify(&self, reader: &mut dyn Read, sig: &[u8]) -> Result<bool> {
        let mut buffer = Vec::new();
        reader.read_to_end(&mut buffer)?;
        let sig = Signature::from_bytes(sig[..64].try_into()?);
        let res = self.key.verify(&buffer, &sig).is_ok();
        Ok(res)
    }
}

impl KeyLoader for Ed25519Verify {
    fn load(key: impl AsRef<Path>) -> Result<Self> {
        let key = fs::read(key)?;
        let key: [u8; 32] = key[..32].try_into()?;
        let key = VerifyingKey::from_bytes(&key)?;
        Ok(Self { key })
    }
}

impl KeyGenerator for Blake3 {
    fn generate() -> Result<Vec<Vec<u8>>> {
        let key = gen_pass::process_genpass(32, true, true, true, true)?;
        Ok(vec![key.as_bytes().to_vec()])
    }
}

impl KeyGenerator for Ed25519Sign {
    fn generate() -> Result<Vec<Vec<u8>>> {
        let mut csprng = OsRng;
        let sk: SigningKey = SigningKey::generate(&mut csprng);
        let pk = sk.verifying_key().to_bytes().to_vec();
        let sk = sk.to_bytes().to_vec();
        Ok(vec![sk, pk])
    }
}

pub fn process_sign(input: &str, key: &str, format: TextSignFormat) -> Result<String> {
    let mut reader: Box<dyn Read> = get_reader(input)?;
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer)?;

    let sign = match format {
        TextSignFormat::Blake3 => {
            // let key = fs::read(key)?;
            // let key: [u8; 32] = key[..32].try_into().unwrap();
            let blake3 = Blake3::load(key)?;
            let sig = blake3.sign(&mut reader)?;
            URL_SAFE_NO_PAD.encode(sig)
        }
        TextSignFormat::Ed25519 => {
            let ed25519 = Ed25519Sign::load(key)?;
            let sig = ed25519.sign(&mut reader)?;
            URL_SAFE_NO_PAD.encode(sig)
        }
    };
    Ok(sign)
}

pub fn process_verify(input: &str, key: &str, sig: String, format: TextSignFormat) -> Result<bool> {
    let mut reader: Box<dyn Read> = get_reader(input)?;
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer)?;

    let res = match format {
        TextSignFormat::Blake3 => {
            let blake3 = Blake3::load(key)?;
            let sig = URL_SAFE_NO_PAD.decode(sig.as_str())?;
            blake3.verify(&mut reader, &sig)?
        }
        TextSignFormat::Ed25519 => {
            let ed25519 = Ed25519Verify::load(key)?;
            let sig = URL_SAFE_NO_PAD.decode(sig.as_str())?;
            ed25519.verify(&mut reader, &sig)?
        }
    };

    Ok(res)
}

pub fn process_generate_key(format: TextSignFormat) -> Result<Vec<Vec<u8>>> {
    match format {
        TextSignFormat::Blake3 => Blake3::generate(),
        TextSignFormat::Ed25519 => Ed25519Sign::generate(),
    }
}

impl Blake3 {
    #[allow(dead_code)]
    pub fn new(key: [u8; 32]) -> Self {
        Self { key }
    }

    #[allow(dead_code)]
    pub fn try_new(key: &[u8]) -> Result<Self> {
        let key: [u8; 32] = key[..32].try_into()?;
        Ok(Self { key })
    }
}

impl Ed25519Sign {
    #[allow(dead_code)]
    pub fn new(key: SigningKey) -> Self {
        Self { key }
    }

    #[allow(dead_code)]
    pub fn try_new(key: &[u8]) -> Result<Self> {
        let key: [u8; 32] = key[..32].try_into()?;
        let key = SigningKey::from_bytes(&key);
        Ok(Self { key })
    }
}

impl Ed25519Verify {
    #[allow(dead_code)]
    pub fn new(key: VerifyingKey) -> Self {
        Self { key }
    }

    #[allow(dead_code)]
    pub fn try_new(key: &[u8]) -> Result<Self> {
        let key: [u8; 32] = key[..32].try_into()?;
        let key = VerifyingKey::from_bytes(&key)?;
        Ok(Self { key })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::*;

    fn get_black3() -> Blake3 {
        let key = Blake3::generate().unwrap();
        let key = key[0].as_slice();
        Blake3 {
            key: key.try_into().unwrap(),
        }
    }
    #[test]
    fn test_blake3_sign() {
        let mut input = "hello world".as_bytes();
        let blake3 = get_black3();
        blake3.sign(&mut input).unwrap();
    }

    #[test]
    fn test_blake3_verify() {
        let input = "hello world";
        let blake3 = get_black3();
        let sign_data = blake3.sign(&mut input.as_bytes()).unwrap();
        let verified = blake3.verify(&mut input.as_bytes(), &sign_data).unwrap();
        assert!(verified);
    }

    #[test]
    fn test_ed25519_sign() {
        let mut input = "hello world".as_bytes();
        let mut rng = thread_rng();
        let mut key: [u8; 32] = [0; 32];
        rng.fill_bytes(&mut key);
        let key = SigningKey::from_bytes(&key);
        let ed25519 = Ed25519Sign { key };
        ed25519.sign(&mut input).unwrap();
    }

    #[test]
    fn test_ed25519_verify() {
        let sk = Ed25519Sign::load("fixtures/ed25519.sk").unwrap();
        let pk = Ed25519Verify::load("fixtures/ed25519.pk").unwrap();
        let input = "hello world";
        let sign_data = sk.sign(&mut input.as_bytes()).unwrap();
        let verified = pk.verify(&mut input.as_bytes(), &sign_data).unwrap();
        assert!(verified);
    }
}
