use serde::Deserialize;
use std::{fs::File, io::BufReader, path::Path};

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct AccountModel {
    pub r#ref: u32,
    pub name: String,
}

pub fn read_default_chart(path: &Path) -> Result<Vec<AccountModel>, String> {
    let file = File::open(path).map_err(|e| e.to_string())?;
    let reader = BufReader::new(file);
    let chart: Vec<AccountModel> = serde_json::from_reader(reader).map_err(|e| e.to_string())?;
    Ok(chart)
}
