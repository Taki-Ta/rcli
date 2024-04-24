use crate::cli::OutputFormat;
use anyhow::Result;
use csv::Reader;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Player {
    pub name: String,
    pub position: String,
    #[serde(rename = "DOB")]
    pub dob: String,
    pub nationality: String,
    #[serde(rename = "Kit Number")]
    pub kit: u8,
}

pub fn process_csv(input: &str, output: String, format: OutputFormat) -> Result<()> {
    let mut reader = Reader::from_path(input)?;
    let headers = reader.headers()?.clone();
    let mut res = Vec::with_capacity(128);
    for i in reader.records() {
        let record = i?;
        let player: Value = headers.iter().zip(record.iter()).collect::<Value>();
        res.push(player);
    }
    let content = match format {
        OutputFormat::Json => serde_json::to_string_pretty(&res)?,
        OutputFormat::Yaml => serde_yaml::to_string(&res)?,
    };
    // let json = serde_json::to_string_pretty(&res)?;
    fs::write(output, content)?;
    Ok(())
}
