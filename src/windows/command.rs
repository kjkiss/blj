use eframe::egui;
use rusqlite::{ params, Connection };

pub struct Command {
    factory: String,
    commands: Vec<String>,
}

pub struct Commands {
    commands: Vec<Command>,
    excelpath: String,
}

impl Default for Commands {
    fn default() -> Self {
        let conn = Connection::open("blj.db").unwrap();
        let mut stmt = conn.prepare("SELECT excelpath FROM store").unwrap();
        let mut store = stmt.query([]).unwrap();

        if let Some(row) = store.next().unwrap() {
            let x: String = row.get(0).unwrap_or_default();
            Self {
                commands: Vec::new(),
                excelpath: x,
            }
        } else {
            Self {
                commands: Vec::new(),
                excelpath: "".into(),
            }
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
        let Self { excelpath, .. } = self;
        ui.label("cisco");
        ui.add(egui::TextEdit::multiline(excelpath).desired_rows(1).hint_text("commands"));
        ui.end_row();
        ui.label("maipu");
        ui.add(egui::TextEdit::multiline(excelpath).desired_rows(1).hint_text("commands"));
        ui.end_row();
        ui.label("h3c");
        ui.add(egui::TextEdit::multiline(excelpath).desired_rows(1).hint_text("commands"));
        ui.end_row();
        ui.label("huawei");
        ui.add(egui::TextEdit::multiline(excelpath).desired_rows(1).hint_text("commands"));
        ui.end_row();
    }
}
