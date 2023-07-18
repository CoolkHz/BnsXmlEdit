#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

pub mod page;
pub mod service;
pub mod util;
use eframe::egui;
use page::main_page::MyApp;

use util::tool::load_icon;

fn main() -> Result<(), eframe::Error> {
    tracing_subscriber::fmt::init();
    // disable_console();

    // let _ = std::io::stdout()
    //     .reopen(std::path::Path::new("NUL"))
    //     .unwrap();

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
        min_window_size: Some(egui::vec2(1700.0, 1000.0)),
        icon_data: Some(load_icon("./static/images/icon.ico")),
        decorated: true,
        ..Default::default()
    };
    eframe::run_native(
        "奇遇xml编辑器",
        options,
        Box::new(|cc| Box::new(MyApp::new(cc))),
    )
}
