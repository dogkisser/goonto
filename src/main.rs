#![windows_subsystem = "windows"]
use std::rc::Rc;
use std::path::Path;
use anyhow::anyhow;
use fltk::app;
use global_hotkey::{GlobalHotKeyManager, GlobalHotKeyEvent, hotkey::{HotKey, Modifiers, Code}};

mod config;
mod sources;
mod features;
use crate::config::Config;

fn dialog(msg: &str) {
    fltk::dialog::message(
        (app::screen_size().0 / 2.0) as i32,
        (app::screen_size().1 / 2.0) as i32, msg);
}

fn main() {
    let singleton = single_instance::SingleInstance::new("zoomasochist-goonto").unwrap();
    if !singleton.is_single() {
        dialog("Goonto is already running.");
        return
    }

    if let Err(e) = Config::load() {
        if e.is::<serde_json::Error>() {
            dialog(&format!("Couldn't parse your configuration file!\n{:?}", e));
            return
        }

        if let Err(e) = Config::default().save() {
            dialog(&format!("Failed to create default config file: {:?}", e));
            return
        }

        dialog("No config file was found, so a default one was created.\n");
    }

    match app() {
        Err(e) => {
            dialog(&format!("Something went wrong :( Please report this to {}\nMessage: {:?}",
                    "https://github.com/zoomasochist/goonto/issues", e));
            },
        _ => { },
    }
}

fn app() -> anyhow::Result<()> {
    let base_dirs = directories::BaseDirs::new()
        .ok_or_else(|| anyhow!("couldn't find base dirs"))?;
    let app_root = base_dirs.data_dir().join("goonto");

    let cfg = Config::load()?;

    let source: Rc<dyn sources::Source> =
        if Path::new(&cfg.source_path).exists() {
            Rc::new(sources::Local::new(cfg.source_path)?)
        } else {
            Rc::new(sources::E621::new(cfg.source_tags)?)
        };

    let manager = GlobalHotKeyManager::new()?;
    let hotkey = HotKey::new(Some(Modifiers::CONTROL), Code::Backspace);
    manager.register(hotkey)?;

    let _app = app::App::default();

    if cfg.popups.enabled {
        cfg.popups.run(Rc::clone(&source));
    }

    if cfg.notifs.enabled {
        cfg.notifs.run(Rc::clone(&source));
    }

    if cfg.typing.enabled {
        cfg.typing.run(Rc::clone(&source));
    }

    if cfg.clipboard.enabled {
        cfg.clipboard.run(Rc::clone(&source));
    }

    loop {
        app::wait_for(100.)?;
        if let Ok(_) = GlobalHotKeyEvent::receiver().try_recv() {
            std::process::exit(0);
        }
    }
}