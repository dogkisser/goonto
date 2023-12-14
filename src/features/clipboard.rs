use std::rc::Rc;
use serde::Deserialize;
use fltk::app;
use defaults::Defaults;

#[derive(Deserialize, Defaults)]
#[serde(default)]
pub struct Clipboard {
    pub enabled: bool,
    #[def = "10_000"]
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