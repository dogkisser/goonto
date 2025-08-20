use std::rc::Rc;
use dialog::DialogBox;
use serde::Deserialize;
use fltk::app;
use defaults::Defaults;

#[derive(Deserialize, Defaults)]
#[serde(default)]
pub struct RepeatDialog {
    pub enabled: bool,
    #[def = "15_000"]
    rate: u128,
    #[def = "true"]
    pub punish: bool,
}

impl RepeatDialog {
    pub fn run<T: crate::sources::Source + 'static + ?Sized>(self, source: Rc<T>) {
        let rate = self.rate as f64 / 1000.;

        app::add_timeout3(rate, move |handle| {
            let prompt = source.first_person();
            let input = dialog::Input::new(format!("\n{prompt}\n"))
                .title("Write this for me, pervert.")
                .show()
                .expect("Could not display dialog box");

            match input {
                Some(ref typed) if typed.trim() == prompt.trim() => {
                    dialog::Message::new("Good gooner, keep sinking deeper.").show();
                }
                Some(_) => {
                    if self.punish {
                        dialog::Message::new("Bad gooner, you're gonna have to make up for it twice.").show();
                    } else {
                        dialog::Message::new("That's not what I asked, try harder next time.").show();
                    }
                }
                None => { }
            }

            app::repeat_timeout3(rate, handle);
        });

    }
}
