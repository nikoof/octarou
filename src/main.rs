// Copyright Nicolas-Ștefan Bratoveanu, 2023,
// licensed under the EUPL-1.2-or-later

use anyhow::Result;
use app::Octarou;
use eframe::egui;
use egui_logger;

mod app;
mod interpreter;

fn main() -> Result<()> {
    egui_logger::init().unwrap();

    for i in 0..1000 {
        log::info!("{}", i);
    }

    const SCALE: f32 = 18.0;
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_min_inner_size([640.0, 320.0])
            .with_inner_size([80.0 * SCALE, 40.0 * SCALE])
            .with_resizable(false),
        ..Default::default()
    };

    let app = Octarou::new();
    eframe::run_native("Octarou", options, Box::new(move |_cc| Box::new(app))).unwrap();

    Ok(())
}
