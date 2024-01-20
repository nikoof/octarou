// Copyright Nicolas-Ștefan Bratoveanu, 2023-2024,
// licensed under the EUPL-1.2-or-later

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use anyhow::Result;
use app::Octarou;

mod app;
mod interpreter;

#[cfg(not(target_arch = "wasm32"))]
fn main() -> Result<()> {
    egui_logger::init().expect("Failed to initialize egui_logger");

    let native_options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_min_inner_size([800.0, 400.0])
            .with_inner_size([1200.0, 600.0])
            .with_resizable(true),
        ..Default::default()
    };

    eframe::run_native(
        "Octarou",
        native_options,
        Box::new(|_cc| Box::new(Octarou::default())),
    )
    .expect("Failed to start eframe");

    Ok(())
}

#[cfg(target_arch = "wasm32")]
fn main() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    let console_log = Box::new(eframe::WebLogger::new(log::LevelFilter::Debug));
    let egui_log = Box::new(egui_logger::EguiLogger);
    multi_log::MultiLogger::init(vec![console_log, egui_log], log::Level::Trace);

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        eframe::WebRunner::new()
            .start(
                "octarou_canvas_id",
                web_options,
                Box::new(|_cc| Box::new(app::Octarou::default())),
            )
            .await
            .expect("Failed to start eframe");
    });
}
