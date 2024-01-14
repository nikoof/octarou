use anyhow::Result;
#[allow(unused_imports)]
use log::{error, info, log, trace, warn};
use std::{fs::File, io::Read, sync::mpsc};

use crate::interpreter::{Chip8, Interpreter, InterpreterError, Superchip};

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
    filename: String,
    data: Vec<u8>,
}

impl Program {
    fn new(filename: impl Into<String>, data: &[u8]) -> Result<Self> {
        Ok(Self {
            filename: filename.into(),
            data: data.to_vec(),
        })
    }
}

pub struct Octarou {
    interpreter: Option<Box<dyn Interpreter>>,
    mode: Mode,
    speed: u64,
    current_program: Option<Program>,

    screen_size: egui::Vec2,
    current_tab: Tab,

    file_dialog_channel: (mpsc::Sender<Program>, mpsc::Receiver<Program>),
}

impl Default for Octarou {
    fn default() -> Self {
        Self {
            interpreter: None,
            mode: Mode::Chip8,
            speed: 700,
            current_program: None,

            screen_size: egui::Vec2::ZERO,
            current_tab: Tab::Controls,

            file_dialog_channel: mpsc::channel(),
        }
    }
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

        if let Some(program) = self.file_dialog_channel.1.try_recv().ok() {
            let filename = program.filename.clone();
            self.current_program = Some(program);
            ctx.send_viewport_cmd(egui::ViewportCommand::Title(format!(
                "Octarou - {}",
                filename
            )));
            self.load_interpreter();
        }

        if let Some(interpreter) = &mut self.interpreter {
            let result = interpreter.tick(&keys_down, &keys_released, self.speed);

            if let Err(e) = result {
                match e {
                    InterpreterError::OutOfMemory => {
                        error!("{}. Stopping execution.", e);
                        self.interpreter = None;
                    }
                    _ => error!("{}.", e),
                }
            }
        }

        self.ui(ctx);
        ctx.request_repaint();
    }
}

impl Octarou {
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
        let task = rfd::AsyncFileDialog::new()
            .add_filter("chip8", &["ch8"])
            .set_directory("/")
            .pick_file();

        let sender = self.file_dialog_channel.0.clone();

        // Hacky way to use RFD (https://github.com/emilk/egui/issues/270)
        // Need this for file dialog in wasm32 builds
        execute(async move {
            let file = task.await;

            if let Some(file) = file {
                #[cfg(not(target_arch = "wasm32"))]
                {
                    let filename = file
                        .path()
                        .file_name()
                        .map(|f| f.to_string_lossy().to_string())
                        .unwrap_or("".to_string());

                    let mut data = Vec::new();
                    let result = File::open(file.path()).map(|mut f| f.read_to_end(&mut data));
                    if let Err(e) = result {
                        error!("{}.", e);
                    }

                    let program = Program::new(filename, &data);
                    match program {
                        Ok(program) => {
                            sender.send(program).ok();
                        }
                        Err(e) => error!("{},", e),
                    }
                }

                #[cfg(target_arch = "wasm32")]
                {
                    let filename = file.inner().name();
                    let data = file.read().await;

                    let program = Program::new(filename, &data);
                    match program {
                        Ok(program) => {
                            sender.send(program).ok();
                        }
                        Err(e) => error!("{},", e),
                    }
                }
            }
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

#[cfg(not(target_arch = "wasm32"))]
fn execute<F: std::future::Future<Output = ()> + Send + 'static>(f: F) {
    std::thread::spawn(move || {
        futures::executor::block_on(f);
    });
}

#[cfg(target_arch = "wasm32")]
fn execute<F: std::future::Future<Output = ()> + 'static>(f: F) {
    wasm_bindgen_futures::spawn_local(f);
}
