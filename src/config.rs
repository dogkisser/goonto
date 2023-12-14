use std::fs::File;
use serde::{Serialize, Deserialize};
use anyhow::Result;

use crate::features::*;

#[derive(Serialize, Deserialize, Default)]
#[serde(default, rename_all = "kebab-case")]
pub struct Config {
    pub run_on_boot: bool,
    pub save_logs: bool,
    pub minimum_run_time: u64,
    pub image_source: ImageSource,
    pub effects: Effects,
    pub babble: Babble,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ImageSource {
    pub web: Web,
    pub local: String,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default, rename_all = "kebab-case")]
pub struct Web {
    pub booru: Booru,
    pub image_res: ImageRes,
    pub tags: Vec<String>,
}

#[derive(Serialize, Deserialize, Default)]
pub enum Booru {
    #[serde(rename = "e621.net")]
    #[default]
    E621,
    #[serde(rename = "rule34.xxx")]
    Rule34,
    #[serde(rename = "realbooru.com")]
    Realbooru,
}

#[derive(Serialize, Deserialize, Default, PartialEq)]
pub enum ImageRes {
    #[serde(rename = "sample")]
    #[default]
    Sample,
    #[serde(rename = "full")]
    Full,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct Effects {
    pub popups: Popups,
    pub notifs: Notifs,
    pub typing: Typing,
    pub clipboard: Clipboard,
    pub discord: Discord,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
#[serde(default)]
pub struct Babble {
    pub first_person: Vec<String>,
    pub third_person: Vec<String>,
}

impl Config {
    pub fn load() -> Result<Self> {
        let path = std::env::current_exe().unwrap().parent().unwrap().join("goonto.yml");
        Ok(serde_yaml::from_reader(File::open(path)?)?)
    }

    pub fn save(&self) -> Result<()> {
        let sample = include_str!("../goonto.yml");
        let path = std::env::current_exe().unwrap().parent().unwrap().join("goonto.yml");
        std::fs::write(path, sample)?;
        Ok(())
    }
}