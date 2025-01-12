#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use dotenv::dotenv;

#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result {
    dotenv().ok();
    env_logger::init();

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 300.0])
            .with_min_inner_size([300.0, 220.0])
            .with_icon(
                eframe::icon_data::from_png_bytes(&include_bytes!("../assets/icon-256.png")[..])
                    .expect("Failed to load icon"),
            ),
        ..Default::default()
    };
    eframe::run_native(
        "rosemary",
        native_options,
        Box::new(|cc| Ok(Box::new(rosemary::Rosemary::new(cc)))),
    )
}
