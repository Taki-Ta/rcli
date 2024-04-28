pub mod b64;
pub mod csv_convert;
pub mod gen_pass;
pub mod http_serve;
pub mod json_web_token;
pub mod text;

pub use b64::*;
pub use csv_convert::*;
pub use gen_pass::*;
pub use http_serve::*;
pub use json_web_token::*;
use std::io::Read;
pub use text::*;

pub trait TextSign {
    fn sign(&self, reader: &mut dyn Read) -> anyhow::Result<Vec<u8>>;
}

pub trait TextVerify {
    fn verify(&self, reader: &mut dyn Read, sig: &[u8]) -> anyhow::Result<bool>;
}
