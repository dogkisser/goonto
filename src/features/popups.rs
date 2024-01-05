use std::rc::Rc;
use std::sync::{Mutex, OnceLock};
use fltk::{prelude::*, app, window::Window, button::Button, image::SharedImage};
use serde::Deserialize;
use rand::Rng;
use defaults::Defaults;

// Not a great implementation but it's really easy and safe
// Update: Deadlocks fixed so far: 1
static COUNT: OnceLock<Mutex<u64>> = OnceLock::new();

#[derive(Deserialize, Copy, Clone, Defaults)]
#[serde(default, rename_all = "kebab-case")]
pub struct Popups {
    #[def = "true"]
    pub enabled: bool,
    #[def = "2_000"]
    rate: u64,
    #[def = "true"]
    closable: bool,
    closes_after: u64,
    max: u64,
    click_through: bool,
    opacity: Opacity,
    mitosis: Mitosis,
}

#[derive(Deserialize, Copy, Clone, Defaults)]
#[serde(default)]
pub struct Mitosis {
    #[def = "30"]
    chance: u16,
    #[def = "5"]
    max: u64,
}

#[derive(Deserialize, Copy, Clone, Defaults)]
#[serde(default)]
pub struct Opacity {
    #[def = "70"]
    from: u16,
    #[def = "100"]
    to: u16,
}

impl Popups {
    pub fn run<T: crate::sources::Source + 'static + ?Sized>(self, source: Rc<T>) {
        let _ = COUNT.get_or_init(|| Mutex::new(0));
        let rate = self.rate as f64 / 1000.;

        app::add_timeout3(rate, move |handle| {
            if let Err(e) = new_popup(Rc::clone(&source), self) {
                log::error!("Couldn't spawn popup: {:?}", e);
            };

            app::repeat_timeout3(rate, handle);
        });
    }
}

/* Returns the number of new popups to create immediately */
fn new_popup<T: crate::sources::Source + 'static + ?Sized>(
    source: Rc<T>,
    cfg: Popups
) -> anyhow::Result<()>
{
    if cfg.max != 0 && *COUNT.get().unwrap().lock().unwrap() >= cfg.max {
        return Ok(())
    }
    
    let image_path = source.image();
    
    if image_path.is_empty() {
        return Ok(())
    }
    
    
    let mut image = SharedImage::load(image_path)?;
    let opacity = rand::thread_rng()
        .gen_range(cfg.opacity.from..=cfg.opacity.to) as f64 / 100.;

    let (img_w, img_h) = reasonable_size(&image);
    let (win_x, win_y) = window_position();
    let mut wind = Window::new(win_x - (img_w / 2), win_y - (img_h / 2), img_w, img_h, "Goonto");
    let mut button = Button::default().with_size(img_w, img_h).center_of_parent();

    image.scale(img_w, img_h, true, true);
    button.set_image(Some(image));

    button.set_callback(move |w| {
        if cfg.closable {
            /* SAFETY: I _know_ this widget has a window */
            w.window().unwrap().hide();
            
            // unsafe {
            //     let image = w.image_mut().unwrap();
            //     <fltk::image::Image as fltk::prelude::ImageExt>::delete(*image);
                // image.delete();
            // }

            {
                let mut c = COUNT.get().unwrap().lock().unwrap();
                *c = c.saturating_sub(1);
            }

            if rand::thread_rng().gen_range(0..100) < cfg.mitosis.chance {
                for _ in 0..rand::thread_rng().gen_range(0..cfg.mitosis.max) {
                    let _ = new_popup(Rc::clone(&source), cfg);
                }
            }
        }
    });

    *COUNT.get().unwrap().lock().unwrap() += 1;

    // wind.set_screen_num(display);
    wind.set_border(false);
    wind.set_callback(|_| { });
    wind.end();
    wind.show();
    wind.set_opacity(opacity);

    make_window_topmost(wind.raw_handle());

    if cfg.click_through {
        make_window_clickthrough(wind.raw_handle());
    }

    if cfg.closes_after > 0 {
        app::add_timeout3(cfg.closes_after as f64 / 1000., move |_handle| {
            wind.hide();
            let mut c = COUNT.get().unwrap().lock().unwrap();
            *c = c.saturating_sub(1);
        });
    }

    Ok(())
}

fn make_window_topmost(handle: fltk::window::RawHandle) {
    #[cfg(target_os = "windows")] unsafe {
        use windows::Win32::Foundation::HWND;
        use windows::Win32::UI::WindowsAndMessaging::{
            SetWindowPos, HWND_TOPMOST, SWP_NOMOVE, SWP_NOSIZE,
        };

        let _ = SetWindowPos(HWND(handle as _), HWND_TOPMOST, 0, 0, 0, 0, SWP_NOMOVE | SWP_NOSIZE);
    }

    #[cfg(target_os = "macos")] unsafe {
        use objc2::{*, runtime::*};

        let wind: &AnyObject = std::mem::transmute::<_, _>(handle);
        let _: () = msg_send![wind, setLevel: (1 as isize)];
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
                display,
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

fn make_window_clickthrough(handle: fltk::window::RawHandle) {
    #[cfg(target_os = "windows")] unsafe {
        use windows::Win32::Foundation::HWND;
        use windows::Win32::UI::WindowsAndMessaging::{
            GWL_EXSTYLE, WS_EX_TRANSPARENT, WS_EX_LAYERED,
            GetWindowLongA, SetWindowLongA
        };

        let current_style = GetWindowLongA(HWND(handle as _), GWL_EXSTYLE) as u32;
        let _ = SetWindowLongA(HWND(handle as _),
            GWL_EXSTYLE,
            (current_style | WS_EX_TRANSPARENT.0 | WS_EX_LAYERED.0) as i32,
        );
    }

    #[cfg(target_os = "linux")] unsafe {
        use x11::{
            xlib::{XRectangle, XOpenDisplay, XCloseDisplay},
            xfixes::{XFixesSetWindowShapeRegion, XFixesDestroyRegion, XFixesCreateRegion},
        };

        let display = XOpenDisplay(std::ptr::null());
        let mut rectangle = XRectangle { x: 0, y: 0, width: 0, height: 0, };
        let region = XFixesCreateRegion(display, &mut rectangle, 1);

        XFixesSetWindowShapeRegion(display, handle, 2, 0, 0, region);
        XFixesDestroyRegion(display, region);
        XCloseDisplay(display);
    }
}

// (x, y, w, h)
fn display_size() -> (i32, i32, i32, i32) {
    let display = rand::thread_rng().gen_range(0..app::screen_count());
    let (x, y, w, h) = app::screen_xywh(display);

    (x, y, w, h)
}

fn window_position() -> (i32, i32) {
    let (x, y, w, h) = display_size();

    (rand::thread_rng().gen_range(x..x+w),
     rand::thread_rng().gen_range(y..y+h))
}

fn reasonable_size(image: &SharedImage) -> (i32, i32) {
    let (_, _, w, h) = display_size();
    let img_w = image.w();
    let img_h = image.h();

    let ratio = f32::min(
        w as f32 / img_w as f32,
        h as f32 / img_h as f32) / 3.;

    ((img_w as f32 * ratio) as i32,
     (img_h as f32 * ratio) as i32)
}
