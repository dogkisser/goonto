use std::rc::Rc;
use serde::{Serialize, Deserialize};
use fltk::{app};
use enigo::{Enigo, KeyboardControllable};

#[derive(Serialize, Deserialize, Debug)]
pub struct Typing {
    pub enabled: bool,
    rate: u128,
}

impl Typing {
    pub fn run<T: crate::sources::Source + 'static + ?Sized>(self, source: Rc<T>) {
        let rate = self.rate as f64 / 1000.;
        let mut enigo = Enigo::new();

        app::add_timeout3(rate, move |handle| {
            enigo.key_sequence_parse(&source.babble());
            
            app::repeat_timeout3(rate, handle);
        });
    }
}

impl Default for Typing {
    fn default() -> Self {
        Self {
            enabled: true,
            rate: 30_000,
        }
    }
}