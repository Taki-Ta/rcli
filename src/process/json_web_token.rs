use crate::get_file_content;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::time::UNIX_EPOCH;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    sub: String,
    aud: String,
    exp: usize,
}

pub fn process_jwt_sign(
    sub: String,
    exp: usize,
    aud: String,
    alg: Algorithm,
) -> anyhow::Result<String> {
    let now = std::time::SystemTime::now()
        .duration_since(UNIX_EPOCH)?
        .as_secs() as usize;
    let claims = Claims {
        sub: sub.to_string(),
        aud: aud.to_string(),
        exp: now + exp,
    };
    let key = get_file_content("fixtures/jwt.key")?;
    let key = key.as_slice();

    let header = Header {
        alg,
        ..Default::default()
    };

    let token = encode(&header, &claims, &EncodingKey::from_secret(key))?;
    Ok(token)
}

pub fn process_jwt_verify(token: String, aud: String, alg: Algorithm) -> anyhow::Result<bool> {
    let key = get_file_content("fixtures/jwt.key")?;
    let key = key.as_slice();

    let mut validation = Validation::new(alg);
    validation.set_audience(&[aud]);
    validation.set_required_spec_claims(&["aud", "exp", "sub"]);

    let data = decode::<Claims>(&token, &DecodingKey::from_secret(key), &validation)?;
    println!("{:?}", data.claims);

    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_jwt_sign_verify() -> anyhow::Result<()> {
        let sub = "taki_1".to_string();
        let exp = 3600;
        let aud = "taki_2".to_string();
        let alg = Algorithm::HS256;
        let token = process_jwt_sign(sub, exp, aud.clone(), alg)?;
        process_jwt_verify(token, aud, alg)?;
        Ok(())
    }
}
