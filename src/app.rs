use crate::chip::Chip8;
use crate::chip::Interpreter;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum InterpreterMode {
    Chip8,
    SuperChip,
}

pub struct Octarou {
    interpreter: Chip8,
    logs: String,
    mode: InterpreterMode,

    screen_size: egui::Vec2,
}

impl eframe::App for Octarou {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.screen_size = ctx.screen_rect().size();
        self.interpreter
            .tick(
                |key| ctx.input(|i| i.key_down(chip8_key_to_egui_key(key).unwrap())),
                || {
                    (0..16).find(|&key| {
                        ctx.input(|i| i.key_released(chip8_key_to_egui_key(key).unwrap()))
                    })
                },
            )
            .unwrap();

        self.ui(ctx);
        ctx.request_repaint();
    }
}

impl Octarou {
    pub fn new(interpreter: Chip8) -> Self {
        Self {
            interpreter,
            mode: InterpreterMode::Chip8,
            logs: "Lorem ipsum dolor sit amet, qui minim labore adipisicing minim sint cillum sint consectetur cupidatat.".to_owned(),

            screen_size: egui::Vec2::ZERO,
        }
    }

    fn ui(&mut self, ctx: &egui::Context) {
        self.menu(ctx);
        self.controls(ctx);
        self.logs(ctx);
        self.interpreter_display(ctx);
    }

    fn menu(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("menubar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
            })
        });
    }

    fn controls(&mut self, ctx: &egui::Context) {
        egui::SidePanel::right("controls")
            .resizable(false)
            .exact_width(0.2 * self.screen_size.x)
            .show(ctx, |ui| {
                ui.with_layout(egui::Layout::default().with_cross_justify(true), |ui| {
                    ui.heading("Interpreter controls");

                    ui.label("Interpreter speed (CPU Cycles / second)");
                    ui.add(egui::Slider::new(&mut self.interpreter.speed, 100..=2000));

                    ui.label("Interpreter mode");
                    egui::ComboBox::from_id_source("mode-selector")
                        .selected_text(format!("{:?}", self.mode))
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut self.mode, InterpreterMode::Chip8, "Chip8");
                            ui.selectable_value(
                                &mut self.mode,
                                InterpreterMode::SuperChip,
                                "SuperChip",
                            );
                        });
                })
            });
    }

    fn logs(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::bottom("logs")
            .exact_height(0.2 * self.screen_size.y)
            .show(ctx, |ui| {
                ui.heading("Logs");
                ui.add(
                    egui::TextEdit::multiline(&mut self.logs.as_str())
                        .code_editor()
                        .desired_rows(1)
                        .desired_width(f32::INFINITY)
                        .lock_focus(true),
                );
            });
    }

    fn interpreter_display(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default()
            .frame(egui::Frame::default().fill(egui::Color32::BLACK))
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    let size = ui.available_size_before_wrap();
                    let scale = egui::vec2(size.x / 64.0, size.y / 32.0);

                    let (response, painter) =
                        ui.allocate_painter(size, egui::Sense::focusable_noninteractive());
                    let rect = response.rect;

                    for (y, row) in self.interpreter.display().iter().enumerate() {
                        for (x, &cell) in row.iter().enumerate() {
                            if cell == 1u8 {
                                let points = [
                                    rect.min
                                        + egui::Vec2::new(x as f32 * scale.x, y as f32 * scale.y),
                                    rect.min
                                        + egui::Vec2::new(
                                            (x + 1) as f32 * scale.x,
                                            (y + 1) as f32 * scale.y,
                                        ),
                                ];
                                painter.rect_filled(
                                    egui::Rect::from_points(&points),
                                    egui::Rounding::ZERO,
                                    egui::Color32::WHITE,
                                );
                            }
                        }
                    }
                });
            });
    }
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
