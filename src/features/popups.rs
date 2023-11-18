use std::rc::Rc;
use fltk::{prelude::*, app, window::Window, button::Button, image::SharedImage};
use serde::{Serialize, Deserialize};
use rand::Rng;

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct Popups {
    pub enabled: bool,
    rate: u64,
    closable: bool,
    opacity: [u16; 2],
    mitosis: Mitosis,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct Mitosis {
    chance: u16,
    max: u64,
}

impl Popups {
    pub fn run<T: crate::sources::Source + 'static + ?Sized>(self, source: Rc<T>) {
        let rate = self.rate as f64 / 1000.;

        app::add_timeout3(rate, move |handle| {
            let _ = new_popup(handle, Rc::clone(&source), self);

            app::repeat_timeout3(rate, handle);
        });
    }
}

impl Default for Popups {
    fn default() -> Self {
        Self {
            enabled: true,
            rate: 1_500,
            closable: true,
            opacity: [30, 100],
            mitosis: Mitosis {
                chance: 20,
                max: 3,
            },
        }
    }
}

/* Returns the number of new popups to create immediately */
fn new_popup<T: crate::sources::Source + 'static + ?Sized>(handle: *mut (), source: Rc<T>, cfg: Popups)
    -> anyhow::Result<()>
{
    let image_path = source.image();

    if image_path.is_empty() {
        return Ok(())
    }
            
    let mut image = SharedImage::load(image_path)?;
    let opacity = rand::thread_rng()
        .gen_range(cfg.opacity[0]..cfg.opacity[1]) as f64 / 100.;

    let (img_w, img_h) = reasonable_size(&image);
    let (win_x, win_y) = window_position();

    let mut wind = Window::new(win_x - img_w / 2, win_y - img_h / 2, img_w, img_h, "Goonto");
    let mut button = Button::default().with_size(img_w, img_h).center_of_parent();

    image.scale(img_w, img_h, true, true);
    button.set_image(Some(image));

    button.set_callback(move |w| {
        if cfg.closable {
            /* SAFETY: I _know_ this widget has a window */
            w.window().unwrap().hide();

            if rand::thread_rng().gen_range(0..100) < cfg.mitosis.chance {
                for _ in 0..rand::thread_rng().gen_range(0..cfg.mitosis.max) {
                    let _ = new_popup(handle, Rc::clone(&source), cfg);
                }
            }
        }
    });

    wind.set_border(false);
    wind.set_callback(|_| { });
    wind.end();
    wind.show();
    wind.set_opacity(opacity);

    make_window_topmost(wind.raw_handle());

    Ok(())
}

fn make_window_topmost(handle: fltk::window::RawHandle) {
    #[cfg(target_os = "windows")] unsafe {
        use winapi::um::winuser::*;

        SetWindowPos(handle as _, HWND_TOPMOST, 0, 0, 0, 0, SWP_NOMOVE | SWP_NOSIZE);
    }

    #[cfg(target_os = "linux")] unsafe {
        use x11::xlib::*;
        use std::ffi::CString;

        let display = XOpenDisplay(&0);

        let wm_state = CString::new(b"_NET_WM_STATE".to_vec()).unwrap();
        let wm_abv = CString::new(b"_NET_WM_STATE_ABOVE".to_vec()).unwrap();

        let mut event = XEvent {
            client_message: XClientMessageEvent {
                type_: ClientMessage,
                serial: 0,
                send_event: 1,
                display: display,
                window: handle,
                message_type: XInternAtom(display, wm_state.as_ptr(), 0),
                format: 32,
                data: ClientMessageData::from([
                    1,
                    XInternAtom(display, wm_abv.as_ptr(), 0),
                    0, 0, 0,
                ]),
            }
        };

        XSendEvent(
            display,
            XDefaultRootWindow(display),
            0,
            SubstructureRedirectMask|SubstructureNotifyMask, &mut event);
        
        XFlush(display);
        XCloseDisplay(display);
    }
}

fn display_size() -> (i32, i32) {
    let mut geometries = Vec::new();
    for i in 0..app::screen_count() {
        geometries.push(app::screen_xywh(i));
    }

    /* SAFETY: All of these vecs contain at least one element, and are ord */
    let x0 = geometries.iter().map(|m| m.0).min().unwrap();
    let y0 = geometries.iter().map(|m| m.1).min().unwrap();
    let x1 = geometries.iter().map(|m| m.0 + m.2).max().unwrap();
    let y1 = geometries.iter().map(|m| m.1 + m.3).max().unwrap();

    (x1 - x0, y1 - y0)
}

fn window_position() -> (i32, i32) {
    let (mon_w, mon_h) = display_size();

    (rand::thread_rng().gen_range(0..mon_w), rand::thread_rng().gen_range(0..mon_h))
}

fn reasonable_size(image: &SharedImage) -> (i32, i32) {
    let (mon_w, mon_h) = display_size();
    let img_w = image.w();
    let img_h = image.h();

    let ratio = f32::min(
        mon_w as f32 / img_w as f32,
        mon_h as f32 / img_h as f32) / 3.;

    ((img_w as f32 * ratio) as i32,
     (img_h as f32 * ratio) as i32)
}