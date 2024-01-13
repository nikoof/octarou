use anyhow::Result;
use egui;
use egui_file::FileDialog;
use std::ffi::OsStr;
use std::io::Read;
use std::path::PathBuf;
use std::{fs::File, path::Path};

use crate::interpreter::{Chip8, Interpreter, Superchip};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Mode {
    Chip8,
    SuperChip,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tab {
    Controls,
    Logs,
}

#[derive(Clone, PartialEq, Eq)]
struct Program {
    file: PathBuf,
    data: Vec<u8>,
}

impl Program {
    fn new(file: &Path) -> Result<Self> {
        let mut data = Vec::new();
        File::open(file)?.read_to_end(&mut data)?;
        let file = file.to_path_buf();

        Ok(Self { file, data })
    }
}

pub struct Octarou {
    interpreter: Option<Box<dyn Interpreter>>,

    mode: Mode,
    speed: u64,

    screen_size: egui::Vec2,
    dialog: Option<FileDialog>,
    open_file: Option<PathBuf>,

    current_program: Option<Program>,

    current_tab: Tab,
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

        self.input(ctx);

        if let Some(interpreter) = &mut self.interpreter {
            interpreter
                .tick(&keys_down, &keys_released, self.speed)
                .unwrap();
        }

        self.ui(ctx);
        ctx.request_repaint();
    }
}

impl Octarou {
    pub fn new() -> Self {
        Self {
            interpreter: None,
            mode: Mode::Chip8,
            speed: 700,

            screen_size: egui::Vec2::ZERO,
            dialog: None,
            open_file: None,

            current_tab: Tab::Controls,
            current_program: None,
        }
    }

    fn load_interpreter(&mut self) {
        if let Some(Program { ref data, .. }) = self.current_program {
            self.interpreter = Some(match self.mode {
                Mode::Chip8 => Box::new(Chip8::new(data)),
                Mode::SuperChip => Box::new(Superchip::new(data)),
            });
        }
    }

    fn input(&mut self, ctx: &egui::Context) {
        ctx.input_mut(|i| {
            if i.consume_key(egui::Modifiers::CTRL, egui::Key::R) {
                self.load_interpreter();
            }
        });
    }

    fn ui(&mut self, ctx: &egui::Context) {
        self.side_panel(ctx);
        self.central_panel(ctx);
        self.file_dialog(ctx);
    }

    fn side_panel(&mut self, ctx: &egui::Context) {
        egui::SidePanel::left("sidepanel")
            .resizable(false)
            .show(ctx, |ui| self.controls(ctx, ui));
    }

    fn controls(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        ui.add_space(6.0);
        ui.vertical_centered_justified(|ui| self.menu(ctx, ui));
        ui.separator();

        egui::Grid::new("controls")
            .num_columns(2)
            .spacing([40.0, 4.0])
            .striped(false)
            .show(ui, |ui| {
                ui.label("Speed:");
                ui.add(egui::Slider::new(&mut self.speed, 100..=2000));
                ui.end_row();

                ui.label("Mode:");
                egui::ComboBox::from_id_source("mode-selector")
                    .selected_text(format!("{:?}", self.mode))
                    .show_ui(ui, |ui| {
                        let chip8 = ui.selectable_value(&mut self.mode, Mode::Chip8, "Chip8");
                        let superchip =
                            ui.selectable_value(&mut self.mode, Mode::SuperChip, "SuperChip");

                        if chip8.clicked() || superchip.clicked() {
                            self.load_interpreter();
                        }
                    });

                ui.end_row();
            });
    }

    fn menu(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        ui.menu_button(egui::RichText::new("\u{2699} Menu").heading(), |ui| {
            if ui.button("Open").clicked() {
                self.open_file_dialog();
                ui.close_menu();
            }

            ui.separator();
            if ui.button("Quit").clicked() {
                ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            }
        });
    }

    fn central_panel(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.selectable_value(
                    &mut self.current_tab,
                    Tab::Controls,
                    egui::RichText::new("\u{1F5A5} Display").heading(),
                );
                ui.selectable_value(
                    &mut self.current_tab,
                    Tab::Logs,
                    egui::RichText::new("\u{1F4DC} Logs").heading(),
                );
            });

            ui.separator();
            match self.current_tab {
                Tab::Controls => self.interpreter_display(ui),
                Tab::Logs => egui_logger::logger_ui(ui),
            }
        });
    }

    fn interpreter_display(&mut self, ui: &mut egui::Ui) {
        egui::Frame::dark_canvas(ui.style()).show(ui, |ui| {
            let rect = ui.available_rect_before_wrap();
            let (response, painter) = ui.allocate_painter(
                if rect.aspect_ratio() < 2.0 {
                    egui::vec2(rect.size().x, 0.5 * rect.size().x)
                } else {
                    egui::vec2(2.0 * rect.size().y, rect.size().y)
                },
                egui::Sense::focusable_noninteractive(),
            );
            self.paint_grid(&painter, response.rect);
        });
    }

    fn paint_grid(&self, painter: &egui::Painter, rect: egui::Rect) {
        if let Some(interpreter) = &self.interpreter {
            let scale = egui::vec2(
                rect.size().x / interpreter.display()[0].len() as f32,
                rect.size().y / interpreter.display().len() as f32,
            );
            for (y, row) in interpreter.display().iter().enumerate() {
                for (x, &cell) in row.iter().enumerate() {
                    if cell == 1u8 {
                        let points = [
                            painter.round_pos_to_pixels(
                                rect.min + egui::Vec2::new(x as f32 * scale.x, y as f32 * scale.y),
                            ),
                            painter.round_pos_to_pixels(
                                rect.min
                                    + egui::Vec2::new(
                                        (x + 1) as f32 * scale.x,
                                        (y + 1) as f32 * scale.y,
                                    ),
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

    fn open_file_dialog(&mut self) {
        let filter = Box::new({
            let ext = Some(OsStr::new("ch8"));
            move |path: &Path| -> bool { path.extension() == ext }
        });
        let mut dialog = FileDialog::open_file(self.open_file.clone()).show_files_filter(filter);
        dialog.open();
        self.dialog = Some(dialog);
    }

    fn file_dialog(&mut self, ctx: &egui::Context) {
        if let Some(dialog) = &mut self.dialog {
            if dialog.show(ctx).selected() {
                if let Some(file) = dialog.path() {
                    self.current_program = Program::new(file).ok();
                    self.load_interpreter();
                }
            }
        }
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
