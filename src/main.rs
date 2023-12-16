#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![warn(clippy::suspicious, clippy::complexity, clippy::style, clippy::perf)]
#![feature(panic_update_hook)]
use std::time::Duration;
use std::{rc::Rc, time::Instant};
use std::path::Path;
use fltk::app;
use global_hotkey::{GlobalHotKeyManager, GlobalHotKeyEvent, hotkey::{HotKey, Modifiers, Code}};
use log::{LevelFilter, error, info};

mod config;
mod sources;
mod features;
use crate::config::Config;

pub fn dialog(msg: &str) {
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

        dialog("No config file was found, so a default one was created. Welcome!\n\
               When you're done, press Control + Backspace to exit.");
    }

    if let Err(e) = app() {
        dialog(&format!("Something went wrong :( Please report this to {}\nMessage: {:?}",
        "https://github.com/zoomasochist/goonto/issues", e));
    }
}

fn app() -> anyhow::Result<()> {
    let started_at = Instant::now();
    let mut cfg = Config::load()?;

    if cfg.save_logs {
        simplelog::WriteLogger::init(
            LevelFilter::Info,
            simplelog::Config::default(),
            std::fs::File::create("goonto.log")?
        )?;

        std::panic::update_hook(move |prev, info| {
            error!("Panic: {:?}", info);
            prev(info);
        });
    } else {
        simplelog::SimpleLogger::init(
            LevelFilter::Info,
            simplelog::Config::default(),
        )?;
    }

    info!("set_run_on_boot: {:?}", set_run_on_boot(cfg.run_on_boot));

    let source: Rc<dyn sources::Source> =
        if Path::new(&cfg.image_source.local).exists() {
            Rc::new(sources::Local::new(&cfg)?)
        } else {
            use crate::config::Booru;

            match cfg.image_source.web.booru {
                Booru::E621 => Rc::new(sources::E621::new(&cfg)?),
                Booru::Rule34 => Rc::new(sources::Rule34::new(&cfg)?),
                Booru::Realbooru => Rc::new(sources::Realbooru::new(&cfg)?),
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

    if cfg.effects.discord.enabled {
        cfg.effects.discord.run(Rc::clone(&source));
    }

    if cfg.effects.wallpaper.enabled {
        cfg.effects.wallpaper.run(Rc::clone(&source));
    }

    loop {
        let min_run_time = Duration::from_millis(cfg.minimum_run_time);
        app::wait_for(100.)?;
        if GlobalHotKeyEvent::receiver().try_recv().is_ok()
            && Instant::now().duration_since(started_at) > min_run_time
        {
            cfg.effects.wallpaper.exit_hook();
            std::process::exit(0);
        }
    }
}

fn set_run_on_boot(to: bool) -> anyhow::Result<()> {
    let directories = directories::BaseDirs::new().unwrap();
    let me = std::env::current_exe()?;
    let cfg = me.parent().unwrap().join("goonto.yml");

    #[cfg(target_os = "linux")] {
        let home = directories.home_dir();
        if to {
            let drop_to = directories.executable_dir().unwrap().join("Goonto");
            let bin_out = drop_to.join("goonto");
            let cfg_out = drop_to.join("goonto.yml");
            
            let desktop = include_str!("../res/linux/Goonto.desktop")
                .replace("{REPLACE_WITH_GOONTO_BIN}",  &bin_out.to_string_lossy())
                .replace("{REPLACE_WITH_GOONTO_PATH}", &drop_to.to_string_lossy());
            
            std::fs::create_dir_all(drop_to)?;
            std::fs::create_dir_all(home.join(".config/autostart"))?;
            
            std::fs::copy(me,  bin_out)?;
            std::fs::copy(cfg, cfg_out)?;
            
            std::fs::write(home.join(".config/autostart/Goonto.desktop"), &desktop)?;
        } else {
            let _ = std::fs::remove_file(home.join(".config/autostart/Goonto.desktop"));
        }
    }

    #[cfg(target_os = "windows")] unsafe {
        use windows::{
            core::{PWSTR, BSTR},
            Win32::{
                Security::PSECURITY_DESCRIPTOR,
                System::{
                    Variant::VariantInit,
                    WindowsProgramming::GetUserNameW,
                    TaskScheduler::{
                        TASK_CREATE_OR_UPDATE, TASK_LOGON_GROUP,
                        TaskScheduler,
                        ITaskService,
                    },
                    Com::{
                        COINIT_MULTITHREADED, RPC_C_AUTHN_LEVEL_PKT_PRIVACY,
                        RPC_C_IMP_LEVEL_IMPERSONATE, EOLE_AUTHENTICATION_CAPABILITIES, CLSCTX_ALL,
                        CoInitializeEx, CoInitializeSecurity, CoCreateInstance
                    }
                }
            }
        };

        let drop_to = directories.config_dir().join("Goonto");
        let bin_out = drop_to.join("goonto.exe");
        let cfg_out = drop_to.join("goonto.yml");

        std::fs::create_dir_all(&drop_to)?;
        
        let mut size: u32 = 100;
        let mut username = Vec::with_capacity(size as usize);
        GetUserNameW(PWSTR(username.as_mut_ptr()), &mut size)?;
        username.set_len(size as usize);

        let username = String::from_utf16(username.as_slice())?;
        let username = username.trim_matches('\0');

        info!("[Run on Boot] using username '{username}' for task");

        let schema = include_str!("../res/win/scheduler-task.xml")
            .replace("{REPLACE_WITH_GOONTO_BIN}", &bin_out.to_string_lossy())
            .replace("{REPLACE_WITH_GOONTO_PATH}", &drop_to.to_string_lossy())
            .replace("{REPLACE_WITH_USERNAME}", username);
        let task_name = BSTR::from("Launch Goonto");
        let task_folder_name = BSTR::from("\\");

        // Initialise COM objects
        CoInitializeEx(None, COINIT_MULTITHREADED)?;
        CoInitializeSecurity(
            PSECURITY_DESCRIPTOR(0 as _),
            -1,
            None,
            None,
            RPC_C_AUTHN_LEVEL_PKT_PRIVACY,
            RPC_C_IMP_LEVEL_IMPERSONATE,
            None,
            EOLE_AUTHENTICATION_CAPABILITIES(0 as _),
            None,
        )?;
        let itsvc: ITaskService = CoCreateInstance(&TaskScheduler, None, CLSCTX_ALL)?;

        itsvc.Connect(std::mem::zeroed(), std::mem::zeroed(), std::mem::zeroed(), std::mem::zeroed())?;
        let task_folder = itsvc.GetFolder(&task_folder_name)?;

        if to {
            std::fs::copy(me, &bin_out)?;
            std::fs::copy(cfg, cfg_out)?;

            // Create task
            let task = itsvc.NewTask(0)?;
            task.SetXmlText(&BSTR::from(schema))?;

            task_folder.RegisterTaskDefinition(
                &task_name,
                &task,
                TASK_CREATE_OR_UPDATE.0,
                std::mem::zeroed(),
                VariantInit(),
                TASK_LOGON_GROUP,
                std::mem::zeroed(),
            )?;
        } else {
            // Ignore the error because it'd probably be that the service doesn't exist
            info!("DeleteTask: {:?}", task_folder.DeleteTask(&task_name, 0));
        }
    }

    Ok(())
}