use std::path::Path;
use std::fs::File;
use serde::{Serialize, Deserialize};
use anyhow::{anyhow, Result};
use directories::BaseDirs;

use crate::features::*;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Config {
    pub source_tags: Vec<String>,
    pub source_path: String,
    pub popups: Popups,
    pub notifs: Notifs,
    pub typing: Typing,
    pub clipboard: Clipboard,
}

impl Config {
    pub fn load() -> Result<Self> {    
        Ok(serde_json::from_reader(File::open("./goonto.json")?)?)
    }

    pub fn save(&self) -> Result<()> {
        std::fs::write("./goonto.json", serde_json::to_string_pretty(&self)?)?;
        Ok(())
    }
}