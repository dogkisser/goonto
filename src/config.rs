use std::path::Path;
use std::fs::File;
use serde::{Serialize, Deserialize};
use anyhow::{anyhow, Result};
use directories::BaseDirs;

use crate::features::*;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Config {
    pub source_tags: Vec<String>,
    pub popups: Popups,
    pub web: Web,
    pub notifs: Notifs,
    pub typing: Typing,
    pub clipboard: Clipboard,
}

impl Config {
    pub fn load() -> Result<Self> {
        let base_dirs = BaseDirs::new().unwrap();

        let possible_paths =
            [ "./goonto.json",
              &base_dirs.home_dir().join(".config/goonto.json").display().to_string()
            ];

        let config =
            possible_paths
                .iter()
                .find(|i| Path::new(i).exists())
                .ok_or_else(|| return anyhow!("no configuration file found"))?;
    
        Ok(serde_json::from_reader(File::open(config)?)?)
    }

    pub fn save(&self) -> Result<()> {
        let base_dirs = BaseDirs::new().unwrap();

        std::fs::create_dir(base_dirs.home_dir().join(".config/"));
        std::fs::write(base_dirs.home_dir().join(".config/goonto.json"),
            serde_json::to_string(&self)?)?;
        Ok(())
    }
}