#[derive(Default)]
pub struct DebugDisplay {}

impl DebugDisplay {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl eframe::App for DebugDisplay {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello Debugger!");
        });
    }
}
