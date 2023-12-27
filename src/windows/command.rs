use eframe::egui;


pub struct Command {
    factory: String,
    commands: Vec<String>,
}

pub struct Commands {
    commands: Vec<Command>,
}

impl Default for Commands {
    fn default() -> Self {
        Self {
            commands: Vec::new()
        }
    }
}

impl super::Window for Commands {
    fn name(&self) -> &'static str {
        "命令"
    }

    fn show(&mut self, ctx: &egui::Context, open: &mut bool) {
        egui::Window
            ::new(self.name())
            .open(open)
            .id(egui::Id::new("command"))
            .resizable(true)
            .default_width(500.0)
            .show(ctx, |ui| {
                use super::View as _;
                self.ui(ui);
            });
    }
}

impl super::View for Commands {
    fn ui(&mut self, ui: &mut egui::Ui) {
        
    }
}
