use std::fs::File;
use serde::{Serialize, Deserialize};
use anyhow::Result;

use crate::features::*;

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "kebab-case")]
pub struct Config {
    pub run_on_boot: bool,
    pub image_source: ImageSource,
    pub effects: Effects,
    pub babble: Babble,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ImageSource {
    pub web: Web,
    pub local: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Web {
    pub booru: Booru,
    pub tags: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub enum Booru {
    #[serde(rename = "e621.net")]
    #[default]
    E621,
    #[serde(rename = "rule34.xxx")]
    Rule34,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Effects {
    pub popups: Popups,
    pub notifs: Notifs,
    pub typing: Typing,
    pub clipboard: Clipboard,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "kebab-case")]
pub struct Babble {
    pub first_person: Vec<String>,
    pub third_person: Vec<String>,
}

impl Config {
    pub fn load() -> Result<Self> {    
        Ok(serde_yaml::from_reader(File::open("./goonto.yml")?)?)
    }

    pub fn save(&self) -> Result<()> {
        std::fs::write("./goonto.yml", serde_yaml::to_string(&self)?)?;
        Ok(())
    }
}