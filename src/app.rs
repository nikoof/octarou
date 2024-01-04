use crate::chip::Chip8;
use crate::chip::Interpreter;

pub struct Octarou {
    interpreter: Chip8,
}

impl Octarou {
    pub fn new(interpreter: Chip8) -> Self {
        Self { interpreter }
    }
}

impl eframe::App for Octarou {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut keys: [bool; 16] = [false; 16];
        for (keyval, key) in [egui::Key::Num0, egui::Key::Num1].into_iter().enumerate() {
            if ctx.input(|i| i.key_down(key)) {
                keys[keyval] = true;
            }
        }
        self.interpreter.tick(&keys).unwrap();

        egui::CentralPanel::default().show(ctx, |ui| {
            for (i, row) in self.interpreter.display.iter().enumerate() {
                for (j, &cell) in row.iter().enumerate() {
                    if cell == 1u8 {
                        let points = [
                            egui::Pos2::new(j as f32 * 10.0, i as f32 * 10.0),
                            egui::Pos2::new((j + 1) as f32 * 10.0, (i + 1) as f32 * 10.0),
                        ];
                        ui.painter().rect_filled(
                            egui::Rect::from_points(&points),
                            egui::Rounding::ZERO,
                            egui::Color32::WHITE,
                        );
                    }
                }
            }
        });
        ctx.request_repaint();
    }
}
