use eframe::egui::{ self };
use rusqlite::{ params, Connection };

#[derive(Debug, Clone)]
pub struct Setting {
    pub username: String,
    pub password: String,
    crtpath: String,
    excelpath: String,
    open_crt_dialog: Option<String>,
    open_excel_dialog: Option<String>,
}

impl Default for Setting {
    fn default() -> Self {
        let conn = Connection::open("blj.db").unwrap();
        let mut stmt = conn
            .prepare("SELECT id, username, password, crtpath, excelpath FROM store")
            .unwrap();
        let mut store = stmt.query([]).unwrap();
        if let Some(row) = store.next().unwrap() {
            Self {
                username: row.get(1).unwrap_or_default(),
                password: row.get(2).unwrap_or_default(),
                crtpath: row.get(3).unwrap_or_default(),
                excelpath: row.get(4).unwrap_or_default(),
                open_crt_dialog: None,
                open_excel_dialog: None,
            }
        } else {
            Self {
                username: Default::default(),
                password: Default::default(),
                crtpath: Default::default(),
                excelpath: Default::default(),
                open_crt_dialog: None,
                open_excel_dialog: None,
            }
        }
    }
}

impl super::Window for Setting {
    fn name(&self) -> &'static str {
        "⚙ 设置"
    }

    fn show(&mut self, ctx: &egui::Context, open: &mut bool) {
        egui::Window
            ::new(self.name())
            .open(open)
            .id(egui::Id::new("window_setting"))
            .resizable(true)
            .default_width(500.0)
            .show(ctx, |ui| {
                use super::View as _;
                self.ui(ui);
            });
    }
}

impl super::View for Setting {
    fn ui(&mut self, ui: &mut egui::Ui) {
        egui::Grid
            ::new("my_grid")
            .num_columns(2)
            .spacing([40.0, 4.0])
            .striped(true)
            .show(ui, |ui| {
                self.gallery_grid_contents(ui);
            });
    }
}

impl Setting {
    fn gallery_grid_contents(&mut self, ui: &mut egui::Ui) {
        let Self { username, password, crtpath, excelpath, .. } = self;

        ui.label("SSH Username");
        ui.add(egui::TextEdit::singleline(username).hint_text("SSH Username"));
        ui.end_row();

        ui.label("SSH Password");
        ui.add(egui::TextEdit::singleline(password).hint_text("SSH Password"));
        ui.end_row();

        ui.label("SecureCRT");
        if ui.text_edit_singleline(crtpath).clicked() {
            if let Some(path) = rfd::FileDialog::new().set_title("选择SecureCRT").pick_file() {
                self.open_crt_dialog = Some(path.display().to_string());
            }
        }
        ui.end_row();

        ui.label("Excelpath");
        if ui.text_edit_singleline(excelpath).clicked() {
            if let Some(path) = rfd::FileDialog::new().set_title("选择Excel").pick_file() {
                self.open_excel_dialog = Some(path.display().to_string());
            }
        }
        ui.end_row();

        if let Some(picked_path) = &self.open_crt_dialog {
            self.crtpath = picked_path.to_string();
        }

        if let Some(picked_path) = &self.open_excel_dialog {
            self.excelpath = picked_path.to_string();
        }

        if ui.button("Save").clicked() {
            let conn = Connection::open("blj.db").unwrap();
            conn.execute(
                "INSERT OR REPLACE INTO store (id, username, password, crtpath, excelpath) VALUES (?1, ?2, ?3, ?4, ?5)",
                params![
                    123,
                    &self.username.clone(),
                    &self.password.clone(),
                    &self.crtpath.clone(),
                    &self.excelpath.clone()
                ]
            ).unwrap();
        }
    }
}
