use anyhow::Result;
use egui;
use egui_file::FileDialog;
use std::ffi::OsStr;
use std::io::Read;
use std::path::PathBuf;
use std::{fs::File, path::Path};

use crate::interpreter::{Chip8, Interpreter, Superchip};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum InterpreterMode {
    Chip8,
    SuperChip,
}

pub struct Octarou {
    interpreter: Option<Box<dyn Interpreter>>,

    mode: InterpreterMode,
    speed: u64,

    screen_size: egui::Vec2,
    open_dialog: Option<FileDialog>,
    open_file: Option<PathBuf>,
}

impl eframe::App for Octarou {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.screen_size = ctx.screen_rect().size();

        let keys_down = (0..16)
            .map(|key| ctx.input(|i| i.key_down(chip8_key_to_egui_key(key).unwrap())))
            .collect::<Vec<bool>>()
            .try_into()
            .expect("Should never panic");

        let keys_released = (0..16)
            .map(|key| ctx.input(|i| i.key_released(chip8_key_to_egui_key(key).unwrap())))
            .collect::<Vec<bool>>()
            .try_into()
            .expect("Should never panic");

        if let Some(interpreter) = &mut self.interpreter {
            interpreter
                .tick(&keys_down, &keys_released, self.speed)
                .unwrap();
        }

        if let Some(dialog) = &mut self.open_dialog {
            if dialog.show(ctx).selected() {
                if let Some(file) = dialog.path() {
                    self.open_file = Some(file.to_path_buf());
                    self.interpreter = Some(Box::new(Chip8::new(Some(
                        &read_program_from_file(file).unwrap(),
                    ))));
                }
            }
        }

        self.ui(ctx);
        ctx.request_repaint();
    }
}

impl Octarou {
    pub fn new() -> Self {
        Self {
            interpreter: None,
            mode: InterpreterMode::Chip8,
            speed: 700,

            screen_size: egui::Vec2::ZERO,
            open_dialog: None,
            open_file: None,
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
                    if ui.button("Open").clicked() {
                        let filter = Box::new({
                            let ext = Some(OsStr::new("ch8"));
                            move |path: &Path| -> bool { path.extension() == ext }
                        });

                        let mut dialog =
                            FileDialog::open_file(self.open_file.clone()).show_files_filter(filter);
                        dialog.open();
                        self.open_dialog = Some(dialog);
                        ui.close_menu();
                    }

                    if ui.button("Quit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
            })
        });
    }

    fn controls(&mut self, ctx: &egui::Context) {
        egui::SidePanel::right("logs")
            .resizable(true)
            .show(ctx, |ui| {
                ui.heading("Logs");
                egui_logger::logger_ui(ui);
            });
    }

    fn logs(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::bottom("controls")
            .exact_height(0.2 * self.screen_size.y)
            .show(ctx, |ui| {
                ui.with_layout(egui::Layout::default().with_cross_justify(true), |ui| {
                    ui.heading("Interpreter controls");

                    ui.label("Interpreter speed (CPU Cycles / second)");
                    ui.add(egui::Slider::new(&mut self.speed, 100..=2000));

                    ui.label("Interpreter mode");

                    // let previous_mode = self.mode;
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

                    // if previous_mode != self.mode {
                    //     let program = self
                    //         .open_file
                    //         .as_ref()
                    //         .map(|path| read_program_from_file(&path).unwrap());
                    //     self.interpreter = match self.mode {
                    //         InterpreterMode::Chip8 => {
                    //             Some(Box::new(Chip8::new(program.as_ref().map(|p| p.as_slice()))))
                    //         }
                    //         InterpreterMode::SuperChip => Some(Box::new(Superchip::new(
                    //             program.as_ref().map(|p| p.as_slice()),
                    //         ))),
                    //     }
                    // }
                });
            });
    }

    fn interpreter_display(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::Frame::default()
                .fill(egui::Color32::BLACK)
                .show(ui, |ui| {
                    ui.vertical_centered(|ui| {
                        let (response, painter) = ui.allocate_painter(
                            ui.available_size_before_wrap(),
                            egui::Sense::focusable_noninteractive(),
                        );
                        self.paint_grid(&painter, response.rect);
                    });
                });
        });
    }

    fn paint_grid(&self, painter: &egui::Painter, rect: egui::Rect) {
        if let Some(interpreter) = &self.interpreter {
            let scale = egui::vec2(rect.size().x / 64.0, rect.size().y / 32.0);
            for (y, row) in interpreter.display().iter().enumerate() {
                for (x, &cell) in row.iter().enumerate() {
                    if cell == 1u8 {
                        let points = [
                            rect.min + egui::Vec2::new(x as f32 * scale.x, y as f32 * scale.y),
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
        }
    }
}

fn read_program_from_file(path: &Path) -> Result<Vec<u8>> {
    let mut buf = Vec::new();
    let mut file = File::open(path)?;
    file.read_to_end(&mut buf)?;
    Ok(buf)
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
