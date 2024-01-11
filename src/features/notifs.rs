use std::rc::Rc;
use serde::Deserialize;
use fltk::app;
use defaults::Defaults;

#[derive(Deserialize, Defaults)]
#[serde(default)]
pub struct Notifs {
    pub enabled: bool,
    #[def = "15_000"]
    rate: u128,
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
                .body(&source.third_person())
                // This doesn't build on Windows?
                // .hint(notify_rust::Hint::Resident(self.close_automatically))
                .show();
            
            app::repeat_timeout3(rate, handle);
        });
    }
}