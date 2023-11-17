use std::rc::Rc;
use serde::{Serialize, Deserialize};
use fltk::{app};

#[derive(Serialize, Deserialize, Debug)]
pub struct Notifs {
    pub enabled: bool,
    rate: u128,
    close_automatically: bool,
}

impl Notifs {
    pub fn run<T: crate::sources::Source + 'static + ?Sized>(self, source: Rc<T>) {
        let rate = self.rate as f64 / 1000.;

        app::add_timeout3(rate, move |handle| {
            /* Errors ignored here.
             * If this fails I doubt there's much I can do about it
             */
            let _ = notify_rust::Notification::new()
                .summary("Goonto notification!")
                .body(&source.prompt())
                // This doesn't build on Windows?
                // .hint(notify_rust::Hint::Resident(self.close_automatically))
                .show();
            
            app::repeat_timeout3(rate, handle);
        });
    }
}

impl Default for Notifs {
    fn default() -> Self {
        Self {
            enabled: true,
            rate: 15_000,
            close_automatically: true,
        }
    }
}