#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::rc::Rc;
use std::path::Path;
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
        if e.is::<serde_yaml::Error>() {
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
    let cfg = Config::load()?;

    set_run_on_boot(cfg.run_on_boot)?;

    let source: Rc<dyn sources::Source> =
        if Path::new(&cfg.image_source.local).exists() {
            Rc::new(sources::Local::new(&cfg)?)
        } else {
            use crate::config::Booru;

            match cfg.image_source.web.booru {
                Booru::E621 => Rc::new(sources::E621::new(&cfg)?),
                Booru::Rule34 => Rc::new(sources::Rule34::new(&cfg)?),
            }
        };

    let manager = GlobalHotKeyManager::new()?;
    let hotkey = HotKey::new(Some(Modifiers::CONTROL), Code::Backspace);
    manager.register(hotkey)?;

    let _app = app::App::default();

    if cfg.effects.popups.enabled {
        cfg.effects.popups.run(Rc::clone(&source));
    }

    if cfg.effects.notifs.enabled {
        cfg.effects.notifs.run(Rc::clone(&source));
    }

    if cfg.effects.typing.enabled {
        cfg.effects.typing.run(Rc::clone(&source));
    }

    if cfg.effects.clipboard.enabled {
        cfg.effects.clipboard.run(Rc::clone(&source));
    }

    loop {
        app::wait_for(100.)?;
        if let Ok(_) = GlobalHotKeyEvent::receiver().try_recv() {
            std::process::exit(0);
        }
    }
}

fn set_run_on_boot(to: bool) -> anyhow::Result<()> {
    let directories = directories::BaseDirs::new().unwrap();

    #[cfg(target_os = "windows")] {
        let startup = directories.config_dir().join("Microsoft/Windows/Start Menu/Programs/Startup");
        let persist_bin = startup.join("goonto.exe");
        let persist_cfg = startup.join("goonto.yml");

        if to {
            let me = std::env::current_exe()?;
            std::fs::copy(me, persist_bin)?;
            std::fs::copy("./goonto.yml", persist_cfg)?;
        } else {
            // Ignore errors because it'll probably be that it doesn't exist, and that's okay.
            let _ = std::fs::remove_file(persist_bin);
            let _ = std::fs::remove_file(persist_cfg);
        }
    }

    Ok(())
}