use crate::{get_reader, TextSign, TextVerify};
use jsonwebtoken::{encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::time::Duration;
use std::{io::Read, string};

#[derive(Debug, Serialize, Deserialize)]
pub struct JWTSigner {
    sub: Option<String>,
    aud: Option<String>,
    exp: Option<Duration>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JWTVerify {}

impl JWTSigner {
    pub fn new(sub: Option<String>, aud: Option<String>, exp: Option<Duration>) -> Self {
        JWTSigner { sub, aud, exp }
    }
}

impl TextSign for JWTSigner {
    fn sign(&self, reader: &mut dyn Read) -> anyhow::Result<Vec<u8>> {
        let mut buffer = Vec::new();
        reader.read_to_end(&mut buffer)?;
        println!("key is {}", std::str::from_utf8(buffer.as_ref())?);

        let token = encode(
            &Header::default(),
            &self,
            &EncodingKey::from_secret(&buffer.as_ref()),
        )
        .unwrap();
        Ok(token.as_bytes().to_vec())
    }
}

impl TextVerify for JWTVerify {
    fn verify(&self, reader: &mut dyn Read, sig: &[u8]) -> anyhow::Result<bool> {
        let mut buffer = Vec::new();
        reader.read_to_end(&mut buffer)?;
        let token = std::str::from_utf8(sig)?;
        let key = std::str::from_utf8(buffer.as_ref())?;
        println!("token2 is {}", token);
        println!("key2 is {:?}", &buffer);
        println!("key is {:?}", key);
        match jsonwebtoken::decode::<JWTSigner>(
            token,
            &DecodingKey::from_secret(&key.as_ref()),
            &Validation::new(Algorithm::HS256),
        ) {
            Ok(_) => Ok(true),
            Err(err) => anyhow::Result::Err(err.into()),
        }
    }
}

pub fn process_jwt_sign(
    key: &str,
    sub: Option<String>,
    aud: Option<String>,
    exp: Option<Duration>,
) -> anyhow::Result<String> {
    let mut reader = get_reader(key)?;
    let signer = JWTSigner::new(sub, aud, exp);
    let sig = signer.sign(&mut reader)?;
    Ok(String::from_utf8(sig)?)
}

pub fn process_jwt_verify(input: &str, token: &str) -> anyhow::Result<bool> {
    let mut reader = get_reader(input)?;
    Ok(JWTVerify {}.verify(&mut reader, token.as_bytes())?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_jwt_sign() {
        let key = "test_key";
        let sub = Some("test_sub".to_string());
        let aud = Some("test_aud".to_string());
        let exp = Some(Duration::from_secs(3600));
        let signer = JWTSigner::new(sub, aud, exp);
        let mut reader = Cursor::new(key);
        let sig = signer.sign(&mut reader).unwrap();
        let token = String::from_utf8(sig).unwrap();
        println!("token is {}", token);
    }

    // #[test]
    // fn test_jwt_verify() {
    //     let key = "test_key";
    //     let sub = Some("test_sub".to_string());
    //     let aud = None;
    //     let exp = Some(Duration::from_secs(3600));
    //     let mut reader=Cursor::new(key);
    //     let signer = JWTSigner::new(sub, aud, exp);
    //     let token=signer.sign(&mut reader).unwrap();
    //     let token=String::from_utf8(token).unwrap();

    //     println!("token1 is {}",token);
    //     println!("key1 is {:?}",key);
    //     let mut reader=Cursor::new(key);
    //     let verify = JWTVerify{};
    //     let res = verify.verify(&mut reader, token.as_bytes()).unwrap();
    //     jsonwebtoken::decode::<JWTSigner>(&token, &DecodingKey::from_secret(key.as_ref()), &Validation::new(Algorithm::HS256));

    // }
}
