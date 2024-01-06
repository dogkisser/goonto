use std::rc::Rc;
use serde::Deserialize;
use fltk::app;
use defaults::Defaults;
use log::info;
#[cfg(target_os = "windows")]
use std::path::PathBuf;
#[cfg(target_os = "windows")]
use windows::{
    core::{HSTRING, PCWSTR},
    Win32::{
        UI::Shell::{IDesktopWallpaper, DesktopWallpaper, DWPOS_TILE, DESKTOP_WALLPAPER_POSITION},
        System::Com::{
            CLSCTX_ALL, COINIT_MULTITHREADED,
            CoCreateInstance, CoInitializeEx,
        },
    },
};

#[derive(Deserialize, Defaults)]
#[serde(default)]
pub struct Wallpaper {
    pub enabled: bool,
    #[def = "10_000"]
    rate: u128,

    #[serde(skip)]
    old_wallpaper: String,
    #[serde(skip)]
    old_wallpaper_position: i32,
}

impl Wallpaper {
    pub fn run<T: crate::sources::Source + 'static + ?Sized>(&mut self, source: Rc<T>) {
        let rate = self.rate as f64 / 1000.;

        if let Err(e) = self.store_old_wallpaper() {
            info!("[Wallpaper]: couldn't store old wallpaper: {:?}", e);
            info!("[Wallpaper]: i won't try to continue.");
            return;
        };

        app::add_timeout3(rate, move |handle| {
            let image = source.image();

            if !image.is_empty() {
                // Winapi doesn't like relative paths
                let canonical = std::path::PathBuf::from(&image)
                    .canonicalize()
                    .unwrap()
                    .to_string_lossy()
                    .to_string();
                
                info!("[Wallpaper] result: {:?}", set_wallpaper(&canonical));
            }
            
            app::repeat_timeout3(rate, handle);
        });
    }
    
    fn store_old_wallpaper(&mut self) -> anyhow::Result<()> {
        #[cfg(target_os = "windows")] unsafe {
            CoInitializeEx(None, COINIT_MULTITHREADED)?;
            let idw: IDesktopWallpaper = CoCreateInstance(&DesktopWallpaper, None, CLSCTX_ALL)?;
            let first_monitor = idw.GetMonitorDevicePathAt(0)?;
    
            let prev_wallpaper_path = PathBuf::from(
                idw.GetWallpaper(&first_monitor.to_hstring()?)?.to_string()?);
    
            let prev_wallpaper_position = idw.GetPosition()?.0;

            info!("[Wallpaper] old wallpaper is: {:?} method {:?}",
                &prev_wallpaper_path,
                prev_wallpaper_position
            );

            self.old_wallpaper = prev_wallpaper_path.to_string_lossy().to_string();
            self.old_wallpaper_position = prev_wallpaper_position;
        }
    
        Ok(())
    }

    pub fn exit_hook(&self) {
        if self.enabled {
            return
        }
        
        info!("[Wallpaper] resetting wallpaper on exit: {:?}",
            set_wallpaper(&self.old_wallpaper));
        
        #[cfg(target_os = "windows")] unsafe {
            let _ = CoInitializeEx(None, COINIT_MULTITHREADED);
            let idw: IDesktopWallpaper = CoCreateInstance(&DesktopWallpaper, None, CLSCTX_ALL)
                .unwrap();
            let _ = idw.SetPosition(DESKTOP_WALLPAPER_POSITION(self.old_wallpaper_position));
        }
    }
}

fn set_wallpaper(to: &str) -> anyhow::Result<()> {
    // For unimplemented platforms
    _ = to;

    #[cfg(target_os = "windows")] unsafe {
        CoInitializeEx(None, COINIT_MULTITHREADED)?;
        let idw: IDesktopWallpaper = CoCreateInstance(&DesktopWallpaper, None, CLSCTX_ALL)?;
        idw.SetWallpaper(PCWSTR::null(), &HSTRING::from(to))?;
        idw.SetPosition(DWPOS_TILE)?;
    }

    Ok(())
}