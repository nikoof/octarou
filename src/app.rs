use crate::chip::Chip8;
use crate::chip::Interpreter;

pub struct Octarou {
    interpreter: Chip8,
}

impl Octarou {
    pub fn new(interpreter: Chip8) -> Self {
        Self { interpreter }
    }

    fn chip8_key_to_egui_key(key: u8) -> Option<egui::Key> {
        match key {
            0 => Some(egui::Key::X),
            1 => Some(egui::Key::Num1),
            2 => Some(egui::Key::Num2),
            3 => Some(egui::Key::Num3),
            4 => Some(egui::Key::Q),
            5 => Some(egui::Key::W),
            6 => Some(egui::Key::E),
            7 => Some(egui::Key::A),
            8 => Some(egui::Key::S),
            9 => Some(egui::Key::D),
            10 => Some(egui::Key::Z),
            11 => Some(egui::Key::C),
            12 => Some(egui::Key::Num4),
            13 => Some(egui::Key::R),
            14 => Some(egui::Key::F),
            15 => Some(egui::Key::V),
            _ => None,
        }
    }
}

impl eframe::App for Octarou {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.interpreter
            .tick(
                |key| ctx.input(|i| i.key_down(Octarou::chip8_key_to_egui_key(key).unwrap())),
                || {
                    (0..16).find(|&key| {
                        ctx.input(|i| i.key_released(Octarou::chip8_key_to_egui_key(key).unwrap()))
                    })
                },
            )
            .unwrap();

        egui::CentralPanel::default().show(ctx, |ui| {
            for (i, row) in self.interpreter.display().iter().enumerate() {
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
