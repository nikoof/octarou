// Copyright Nicolas-Ștefan Bratoveanu, 2023,
// licensed under the EUPL-1.2-or-later

use anyhow::Result;
use app::Octarou;

mod app;
mod interpreter;

fn main() -> Result<()> {
    egui_logger::init().unwrap();

    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_min_inner_size([800.0, 400.0])
            .with_inner_size([1200.0, 600.0])
            .with_resizable(true),
        ..Default::default()
    };

    let app = Octarou::new();
    eframe::run_native("Octarou", options, Box::new(move |_cc| Box::new(app))).unwrap();

    Ok(())
}
