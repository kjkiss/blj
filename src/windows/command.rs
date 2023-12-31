use eframe::egui;
use rusqlite::{ Connection, params };
use crate::model::switch::Switch;
use std::{ iter::zip, collections::HashSet };

#[derive(Debug)]
pub struct Commands {
    factory: Vec<String>,
    commands: Vec<String>,
}

impl Default for Commands {
    fn default() -> Self {

        let mut factorys = Vec::new();
        let mut commands = Vec::new();

        let conn = Connection::open("blj.db").unwrap();
        let mut stmt = conn.prepare("SELECT factory, command FROM commands").unwrap();

        let mut store = stmt.query([]).unwrap();

        while let Some(row) = store.next().unwrap() {
            let m: String = row.get(0).unwrap_or_default();
            let n: String = row.get(1).unwrap_or_default();
            factorys.push(m);
            commands.push(n);
        }

        if factorys.len() > 0 {
            return  Self {
                factory: factorys,
                commands,
            };
        }

        let factorys = Switch::get_factory();
        let mut a = factorys.0;
        let b = factorys.1;
        a.extend(b);
        let c: HashSet<String> = HashSet::from_iter(a);

        let kk: Vec<String> = Vec::from_iter(c);

        let len = kk.len();
        let y = (0..len).map(|_x| "".to_string()).collect::<Vec<String>>();
        Self {
            factory: kk,
            commands: y,
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
        let Self { factory, commands } = self;

        let mut z = zip(factory, commands);

        for (name, j) in &mut z {
            ui.label(name.clone());
            ui.add(egui::TextEdit::multiline(j).desired_rows(1).hint_text("commands"));
            ui.end_row();
        }

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
