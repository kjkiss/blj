use eframe::egui;

pub struct Command {
    factory: String,
    commands: Vec<String>,
}

pub struct Commands {
    commands: Vec<Command>,
    text: String,
}

impl Default for Commands {
    fn default() -> Self {
        Self {
            commands: Vec::new(),
            text: "".into(),
        }
    }
}

impl super::Window for Commands {
    fn name(&self) -> &'static str {
        "⛱ 命令"
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
        egui::Grid
            ::new("commands_grid")
            .num_columns(2)
            .spacing([40.0, 4.0])
            .striped(true)
            .show(ui, |ui| {
                self.gallery_grid_contents(ui);
            });
    }
}

impl Commands {
    fn gallery_grid_contents(&mut self, ui: &mut egui::Ui) {
        let Self { text, .. } = self;
        ui.label("cisco");
        ui.add(egui::TextEdit::multiline(text).desired_rows(1).hint_text("commands"));
        ui.end_row();
        ui.label("maipu");
        ui.add(egui::TextEdit::multiline(text).desired_rows(1).hint_text("commands"));
        ui.end_row();
        ui.label("h3c");
        ui.add(egui::TextEdit::multiline(text).desired_rows(1).hint_text("commands"));
        ui.end_row();
        ui.label("huawei");
        ui.add(egui::TextEdit::multiline(text).desired_rows(1).hint_text("commands"));
        ui.end_row();
    }
}
