use std::rc::Rc;
use serde::{Serialize, Deserialize};
use fltk::{app};

#[derive(Serialize, Deserialize, Debug)]
pub struct Web {
    pub enabled: bool,
    rate: u128,
}

impl Web {
    pub fn run(self, source: Rc<crate::source::Source>) {
        let rate = self.rate as f64 / 1000.;

        app::add_timeout3(rate, move |handle| {
            open_page(source.url());

            app::repeat_timeout3(rate, handle);
        });
    }
}

impl Default for Web {
    fn default() -> Self {
        Self {
            enabled: true,
            rate: 3_000,
        }
    }
}

fn open_page(url: String) {
    #[cfg(target_os = "windows")] std::process::Command::new("rundll32")
        .args(["url.dll,FileProtocolHandler", &url]).spawn();
    #[cfg(target_os = "linux")] std::process::Command::new("xdg-open").args([url]).spawn();
    #[cfg(target_os = "macos")] std::process::Command::new("open").args([url]).spawn();
}