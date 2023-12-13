use std::rc::Rc;
use serde::{Serialize, Deserialize};
use fltk::{app};
use enigo::{Enigo, KeyboardControllable, Key};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct Typing {
    pub enabled: bool,
    rate: u128,
    press_enter: bool,
}

impl Typing {
    pub fn run<T: crate::sources::Source + 'static + ?Sized>(self, source: Rc<T>) {
        let rate = self.rate as f64 / 1000.;
        let mut enigo = Enigo::new();

        app::add_timeout3(rate, move |handle| {
            enigo.key_sequence_parse(&source.first_person());
            if self.press_enter {
                enigo.key_click(Key::Return);
            }

            app::repeat_timeout3(rate, handle);
        });
    }
}

impl Default for Typing {
    fn default() -> Self {
        Self {
            enabled: true,
            rate: 30_000,
            press_enter: false,
        }
    }
}