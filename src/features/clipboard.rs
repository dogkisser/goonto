use std::rc::Rc;
use serde::{Serialize, Deserialize};
use fltk::{app};

#[derive(Serialize, Deserialize, Debug)]
pub struct Clipboard {
    pub enabled: bool,
    rate: u128,
}

impl Clipboard {
    pub fn run<T: crate::sources::Source + 'static + ?Sized>(self, source: Rc<T>) {
        let rate = self.rate as f64 / 1000.;

        app::add_timeout3(rate, move |handle| {
            app::copy(&source.first_person());
            
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