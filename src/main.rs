// Copyright Nicolas-Ștefan Bratoveanu, 2023,
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
    egui_logger::init().expect("Failed to initialize egui_logger");
    eframe::WebLogger::init(log::LevelFilter::Debug)
        .expect("Failed to initialize eframe::WebLogger");

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
