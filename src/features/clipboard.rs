use std::rc::Rc;
use serde::{Serialize, Deserialize};
use fltk::{app};

#[derive(Serialize, Deserialize, Debug)]
pub struct Clipboard {
    pub enabled: bool,
    rate: u128,
}

impl Clipboard {
    pub fn run(self, source: Rc<crate::source::Source>) {
        let rate = self.rate as f64 / 1000.;

        app::add_timeout3(rate, move |handle| {
            app::copy(&source.prompt());
            
            app::repeat_timeout3(rate, handle);
        });
    }
}

impl Default for Clipboard {
    fn default() -> Self {
        Self {
            enabled: true,
            rate: 10_000,
        }
    }
}