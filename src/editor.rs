use std::sync::{RwLock, Arc};
use crate::config::Config;
use fltk_theme::{ColorTheme, color_themes, SchemeType, WidgetScheme};
use fltk_grid::Grid;
use fltk::{
    prelude::{*, WidgetExt},
    app,
    input::Input,
    button::{CheckButton, Button}, window::Window, group::{Flex, Tabs},
};

pub fn start() {
    let config: Arc<RwLock<Config>>
        = Arc::new(Config::load().unwrap_or_else(|_| Config::default()).into());

    let _app = app::App::default()
        .with_scheme(fltk::app::Scheme::Gtk);

    let theme = ColorTheme::new(color_themes::BLACK_THEME);
    let scheme = WidgetScheme::new(SchemeType::Aqua);
    theme.apply();
    scheme.apply();

    let mut wind = Window::default()
        .with_size(500, 450)
        .with_label("Goonto Editor");

    let mut tab = Tabs::default_fill();
    
    misc_group(Arc::clone(&config));
    popup_group(Arc::clone(&config));

    tab.end();
    tab.auto_layout();

    Button::default().with_label("Save");

    wind.make_resizable(true);
    wind.end();
    wind.show();

    app::run().unwrap();
}

fn misc_group(cfg: Arc<RwLock<Config>>) {
    let group = Flex::default_fill().with_label("Misc").row();

    let mut grid = Grid::default_fill();
    grid.set_layout(5, 2);

    let mut b = CheckButton::default().with_label("Run on boot");
    b.set_checked(cfg.read().unwrap().run_on_boot);
    let c = Arc::clone(&cfg);
    b.set_callback(move |state| {
        c.write().unwrap().run_on_boot = state.is_checked();
    });
    grid.set_widget(&mut b, 0, 0);

    grid.end();
    group.end();
}

fn popup_group(cfg: Arc<RwLock<Config>>) {
    let mut group = Flex::default_fill().with_label("Popups").row();
    let mut col = Flex::default().column();
    group.fixed(&col, 160);
    col.set_pad(10);
    col.set_margin(10);

    let mut button = Button::default().with_label("t");

    button.set_callback(|b|  {
        println!("clicked!");
    });

    col.end();
    group.end();
}