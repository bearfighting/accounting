use serde::Deserialize;
use std::path::Path;
use tokio::{fs::File, io::BufReader};

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct AccountModel {
    pub r#ref: u32,
    pub name: String,
}

pub async fn read_default_chart(path: &Path) -> Result<Vec<AccountModel>, String> {
    let file = File::open(path).await.map_err(|e| e.to_string())?;
    let mut reader = BufReader::new(file);
    let mut contents = String::new();
    use tokio::io::AsyncReadExt;
    reader
        .read_to_string(&mut contents)
        .await
        .map_err(|e| e.to_string())?;
    let chart: Vec<AccountModel> = serde_json::from_str(&contents).map_err(|e| e.to_string())?;
    Ok(chart)
}
