use eframe::egui;
use rusqlite::Connection;
use crate::model::switch::Switch;
use std::{ iter::zip, collections::HashSet };

#[derive(Debug)]
pub struct Commands {
    pub factory: Vec<String>,
    pub commands: Vec<String>,
}

impl Default for Commands {
    fn default() -> Self {
        let mut factorys = Vec::new();
        let mut commands = Vec::new();

        let conn = Connection::open("blj.db").unwrap();
        let mut stmt = conn.prepare("SELECT factory, command FROM commands").unwrap();

        let mut store = stmt.query([]).unwrap();

        while let Some(row) = store.next().unwrap() {
            let factory: String = row.get(0).unwrap_or_default();
            let command: String = row.get(1).unwrap_or_default();
            factorys.push(factory);
            commands.push(command);
        }

        if factorys.len() > 0 {
            return Self {
                factory: factorys,
                commands,
            };
        }

        let factorys = Switch::get_factory();
        let mut intranet_factory = factorys.0;
        let internet_factory = factorys.1;
        intranet_factory.extend(internet_factory);
        let factory_set: HashSet<String> = HashSet::from_iter(intranet_factory);

        let all_factory: Vec<String> = Vec::from_iter(factory_set);

        let len = all_factory.len();
        let commands = (0..len).map(|_x| "".to_string()).collect::<Vec<String>>();
        Self {
            factory: all_factory,
            commands,
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
        ui.add_space(10.0);
        self.add_button(ui);
    }
}

impl Commands {
    fn gallery_grid_contents(&mut self, ui: &mut egui::Ui) {
        let Self { factory, commands } = self;

        let mut z = zip(factory, commands);
        for (name, j) in &mut z {
            ui.label(name.clone());
            ui.add(egui::TextEdit::multiline(j).desired_rows(1).hint_text("commands"));
            ui.end_row();
        }
    }

    fn add_button(&mut self, ui: &mut egui::Ui) {
        if ui.button("Save").clicked() {
            let conn = Connection::open("blj.db").unwrap();
            let mut stmt = conn
                .prepare("INSERT OR REPLACE INTO commands (factory, command) VALUES (?1, ?2)")
                .unwrap();
            let factory = self.factory.clone();
            let commands = self.commands.clone();

            for (factory, commands) in zip(factory, commands) {
                stmt.execute((factory, commands)).unwrap();
            }
        }
    }
}
