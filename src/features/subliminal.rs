use std::{rc::Rc, sync::Arc};
use rand::Rng;
use serde::Deserialize;
use defaults::Defaults;
use fltk::{app,
    window::Window,
    prelude::{WidgetBase, WidgetExt, WindowExt, GroupExt},
    frame::Frame, surface, image, enums, draw};

#[derive(Deserialize, Defaults)]
#[serde(default, rename_all = "kebab-case")]
pub struct Subliminal {
    pub enabled: bool,
    #[def = "10_000"]
    rate: u128,
    opacity: Opacity,
    monitors: Monitors,
}

#[derive(Deserialize, Defaults)]
#[serde(default)]
pub struct Opacity {
    #[def = "20"]
    from: u16,
    #[def = "50"]
    to: u16,
}

#[derive(Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub enum Monitors {
    All,
    #[default]
    Primary,
    ExceptPrimary,
    #[serde(untagged)]
    These(Vec<i32>),
}

impl Subliminal {
    pub fn run<T: crate::sources::Source + 'static + ?Sized>(self, source: Rc<T>) {
        let rate = self.rate as f64 / 1000.;

        let subliminal = Arc::new(self);
        app::add_timeout3(rate, move |handle| {
            let source = Rc::clone(&source);
            let subliminal = Arc::clone(&subliminal);
            if let Err(e) = new_subliminal(source, subliminal) {
                log::warn!("Couldn't spawn subliminal: {e:?}");
            }

            // app::repeat_timeout3(rate, handle);
        });
    }
}

fn new_subliminal<T: crate::sources::Source + 'static + ?Sized>(
    source: Rc<T>,
    cfg: Arc<Subliminal>,
) -> anyhow::Result<()>
{
    for (x, y, w, h) in get_monitors(&cfg) {
        let wind_height = h / 3;
        let wind_y = (y + h / 2) - wind_height / 2;

        let mut wind = Window::new(x, wind_y, w, wind_height, "Goonto Subliminal");
        let mut text = Frame::default_fill().with_label("Test");

        let opacity = rand::thread_rng()
            .gen_range(cfg.opacity.from..=cfg.opacity.to) as f64 / 100.;

        wind.set_border(false);
        wind.set_callback(|_| { });
        wind.end();
        wind.show();

        let cardinal_alpha = u32::MAX * (opacity as u32);

        unsafe {
            let xid = wind.raw_handle();

            let display = x11::xlib::XOpenDisplay(&0);
            let atom_name = std::ffi::CString::new("_NET_WM_WINDOW_OPACITY")?;
            let atom = x11::xlib::XInternAtom(display, atom_name.as_ptr(), 0);
            x11::xlib::XChangeProperty(
                display,
                xid,
                atom,
                x11::xlib::XA_CARDINAL,
                32,
                x11::xlib::PropModeReplace,
                std::ptr::addr_of!(cardinal_alpha) as *const u8,
                1,
            );

            println!("sent msg");
        }
        // wind.set_opacity(opacity);
    }

    Ok(())
}

fn get_monitors(cfg: &Subliminal) -> Vec<(i32, i32, i32, i32)> {
    let screen_count = app::screen_count();

    match (&cfg.monitors, screen_count) {
        (_, 1) => vec![0, 1],
        (Monitors::All, x) => (0..x).collect(),
        (Monitors::Primary, _) => vec![0, 1],
        (Monitors::ExceptPrimary, _) => (1..screen_count).collect(),
        (Monitors::These(x), _) => x.clone(),
    }
        .iter()
        .map(|m| app::screen_xywh(*m))
        .collect()
}