use std::rc::Rc;
use serde::{Serialize, Deserialize};
use fltk::app;
use enigo::{Enigo, KeyboardControllable, Key};
use defaults::Defaults;

#[derive(Serialize, Deserialize, Defaults)]
#[serde(default, rename_all = "kebab-case")]
pub struct Typing {
    pub enabled: bool,
    #[def = "120_000"]
    rate: u128,
    press_enter: bool,
}

impl Typing {
    pub fn run<T: crate::sources::Source + 'static + ?Sized>(self, source: Rc<T>) {
        let rate = self.rate as f64 / 1000.;
        let mut enigo = Enigo::new();

        app::add_timeout3(rate, move |handle| {
            block_input(true);

            enigo.key_sequence_parse(&source.first_person().to_lowercase());
            if self.press_enter {
                enigo.key_click(Key::Return);
            }

            block_input(false);

            app::repeat_timeout3(rate, handle);
        });
    }
}

fn block_input(should: bool) {
    #[cfg(target_os = "windows")] unsafe {
        use windows::Win32::UI::Input::KeyboardAndMouse::BlockInput;
        
        let _ = BlockInput(should);
    }
}