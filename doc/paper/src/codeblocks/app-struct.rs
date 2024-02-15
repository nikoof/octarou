pub struct Octarou {
    interpreter: Option<Box<dyn Interpreter>>,
    mode: Mode,
    speed: u64,
    current_program: Option<Program>,

    screen_size: egui::Vec2,
    current_tab: Tab,

    file_dialog_channel: (mpsc::Sender<Program>, mpsc::Receiver<Program>),

    #[allow(unused)]
    stream: (rodio::OutputStream, rodio::OutputStreamHandle),
    sink: rodio::Sink,
    muted: bool,
}
