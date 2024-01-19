use std::fs::File;
use serde::Deserialize;
use anyhow::Result;
use defaults::Defaults;
use fltk::dialog::{self, FileDialogType};

use crate::features::*;

#[derive(Deserialize, Defaults)]
#[serde(default, rename_all = "kebab-case")]
pub struct Config {
    pub run_on_boot: bool,
    pub save_logs: bool,
    pub minimum_run_time: u64,
    pub image_source: ImageSource,
    pub effects: Effects,
    pub babble: Babble,
    #[def = "String::from(\"Ctrl + Backspace\")"]
    pub exit_keybind: String,
}

#[derive(Deserialize, Default)]
#[serde(default)]
pub struct ImageSource {
    pub web: Web,
    pub local: String,
}

#[derive(Deserialize, Defaults)]
#[serde(default, rename_all = "kebab-case")]
pub struct Web {
    pub booru: Booru,
    pub image_res: ImageRes,
    #[def = "String::from(\"rating:explicit score:>200\")"]
    pub tag_prefix: String,
    pub tags: Vec<String>,
}

#[derive(Deserialize, Default)]
pub enum Booru {
    #[serde(rename = "e621.net")]
    #[default]
    E621,
    #[serde(rename = "rule34.xxx")]
    Rule34,
    #[serde(rename = "realbooru.com")]
    Realbooru,
}

#[derive(Deserialize, Default, PartialEq, Debug)]
pub enum ImageRes {
    #[serde(rename = "sample")]
    #[default]
    Sample,
    #[serde(rename = "full")]
    Full,
}

#[derive(Deserialize, Default)]
#[serde(default)]
pub struct Effects {
    pub popups: Popups,
    pub notifs: Notifs,
    pub typing: Typing,
    pub clipboard: Clipboard,
    pub discord: Discord,
    pub wallpaper: Wallpaper,
}

#[derive(Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
#[serde(default)]
pub struct Babble {
    pub first_person: Vec<String>,
    pub third_person: Vec<String>,
}

impl Config {
    pub fn load() -> Result<Self> {
        let bin = std::env::current_exe().unwrap();
        let dir = bin.parent().unwrap();
        let yml_files = std::fs::read_dir(dir)?
            .filter(|f| f
                    .as_ref()
                    .unwrap()
                    .path()
                    .extension()
                    .is_some_and(|e| e == "yml")
            )
            .map(|f| f.unwrap().path())
            .collect::<Vec<std::path::PathBuf>>();
        anyhow::ensure!(!yml_files.is_empty(), "No configuration files found");

        if yml_files.len() == 1 {
            return Ok(serde_yaml::from_reader(File::open(&yml_files[0])?)?);
        }

        let mut nfc = dialog::NativeFileChooser::new(FileDialogType::BrowseFile);
        nfc.set_directory(&dir)?;
        nfc.set_title("Choose Config file");
        nfc.set_filter("*.yml");
        nfc.show();

        match nfc.filename() {
            x if x.as_os_str().is_empty() => {
                crate::dialog("You didn't choose a config file! Exiting.");
                std::process::exit(1);
            },
            sel => Ok(serde_yaml::from_reader(File::open(sel)?)?),
        }
    }

    pub fn save(&self) -> Result<()> {
        let sample = include_str!("../res/goonto.yml");
        let path = std::env::current_exe().unwrap().parent().unwrap().join("goonto.yml");
        std::fs::write(path, sample)?;
        Ok(())
    }
}