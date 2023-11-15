#![windows_subsystem = "windows"]
use std::rc::Rc;
use anyhow::anyhow;
use clap::Parser;
use fltk::{prelude::*, app, window::Window, group::Flex, group::Pack, frame::Frame, button::CheckButton};
use fltk::enums::Color;
use global_hotkey::{GlobalHotKeyManager, GlobalHotKeyEvent, hotkey::{HotKey, Modifiers, Code}};

mod config;
mod source;
mod features;
use crate::config::Config;
use crate::features::*;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// Launch the fetish wizard and exit
    #[arg(long)]
    wizard: bool,
}

fn dialog(msg: &str) {
    fltk::dialog::message(
        (app::screen_size().0 / 2.0) as i32,
        (app::screen_size().1 / 2.0) as i32, msg);
}

fn main() {
    let args = Args::parse();

    if args.wizard {
        wizard();
        return
    }

    if let Err(_) = Config::load() {
        if let Err(e) = Config::default().save() {
            dialog(&format!("Failed to create default config file: {:?}", e));
            return
        }

        dialog("No config file was found, so a default one was created. \
            You can use the wizard to edit it.\n");
    }

    match app() {
        Err(e) => {
            dialog(&format!("Something went wrong :( Please report this to {}\nMessage: {:?}",
                    "https://github.com/zoomasochist/goonto/issues", e));
            },
        _ => { },
    }
}

fn wizard() {
    // let app = app::App::default().with_scheme(app::Scheme::Gtk);
    // app::background(255, 255, 255);

    // let mut wind = Window::default()
    //     .with_size(300, 200)
    //     .with_label("Goonto Wizard")
    //     .center_screen();
    
    // Frame::default().with_size(300, 200)
    //     .with_label("There's nothing here yet.");
    // let mut rows = Flex::default_fill().row().center_of_parent();

    // let mut meta_row = Flex::default().column();
    // {
    //     Frame::default();

    //     let ros = CheckButton::default().with_label("Run on startup");

    //     meta_row.fixed(&ros, 80);
    // }
    // meta_row.end();

    // col.fixed(&meta_flex, 200);

    // let mut text = Frame::default()
    //     .with_size(300, 200)
    //     .with_label("Select things you'd like to see"); 

    // rows.end();
    // wind.end();
    // wind.show();

    // app.run().unwrap();
}

fn app() -> anyhow::Result<()> {
    let base_dirs = directories::BaseDirs::new()
        .ok_or_else(|| anyhow!("couldn't find base dirs"))?;
    let app_root = base_dirs.data_dir().join("goonto");

    let cfg = Config::load()?;
    let source = Rc::new(source::Source::new(cfg.source_tags, app_root)?);

    let manager = GlobalHotKeyManager::new()?;
    let hotkey = HotKey::new(Some(Modifiers::CONTROL), Code::Backspace);
    manager.register(hotkey);

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

    if cfg.web.enabled {
        cfg.web.run(Rc::clone(&source));
    }

    loop {
        app::wait_for(100.)?;
        if let Ok(evt) = GlobalHotKeyEvent::receiver().try_recv() {
            std::process::exit(0);
        }
    }
}