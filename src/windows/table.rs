use eframe::egui;
use egui_extras::{ TableBuilder, Column };

use calamine::{ Reader, Xlsx, open_workbook };

use crate::model::switch::Switch;
use crate::model::blj::Kind;

pub struct Table {
    switchs: (Vec<Switch>, Vec<Switch>),
    open_excel_dialog: Option<String>,
    excel_path: String,
    kind: Kind,
}

impl Default for Table {
    fn default() -> Self {
        let table = Table::new();
        let switchs = get_excel(table.excel_path);
        Table {
            switchs,
            open_excel_dialog: None,
            excel_path: "".to_string(),
            kind: Kind::Intranet,
        }
    }
}

impl Table {
    fn new() -> Self {
        Self {
            switchs: (Vec::new(), Vec::new()),
            open_excel_dialog: None,
            excel_path: "".to_string(),
            kind: Kind::Intranet,
        }
    }
}

fn get_excel(excel_path: String) -> (Vec<Switch>, Vec<Switch>) {
    if excel_path.is_empty() {
        return (vec![], vec![]);
    }

    let mut wb: Xlsx<_> = open_workbook(excel_path).expect("open xlsx err");
    let mut intranet = vec![];
    let mut internet = vec![];

    let sheets = wb.sheet_names();

    if let Some(Ok(r)) = wb.worksheet_range(sheets[0].as_str()) {
        for row in r.rows().skip(1) {
            intranet.push(Switch {
                area: row[0].to_string(),
                name: row[1].to_string(),
                model: row[2].to_string(),
                factory: row[3].to_string().trim().to_lowercase(),
                ip: row[4].to_string().trim().into(),
                port: row[5].to_string().trim().into(),
                floor: row[6].to_string(),
            });
        }
    }

    if let Some(Ok(r)) = wb.worksheet_range(sheets[1].as_str()) {
        for row in r.rows().skip(1) {
            internet.push(Switch {
                area: row[0].to_string(),
                name: row[1].to_string(),
                model: row[2].to_string(),
                factory: row[3].to_string().trim().to_lowercase(),
                ip: row[4].to_string().trim().into(),
                port: row[5].to_string().trim().into(),
                floor: row[6].to_string(),
            });
        }
    }

    (intranet, internet)
}

impl super::Window for Table {
    fn name(&self) -> &'static str {
        "☰ 设备表"
    }

    fn show(&mut self, ctx: &eframe::egui::Context, open: &mut bool) {
        egui::Window
            ::new(self.name())
            .open(open)
            .resizable(true)
            .default_width(500.0)
            .show(ctx, |ui| {
                use super::View as _;
                self.ui(ui);
            });
    }
}

impl super::View for Table {
    fn ui(&mut self, ui: &mut egui::Ui) {
        let Self { excel_path, .. } = self;
        ui.vertical(|ui| {
            ui.add_space(15.0);
            ui.horizontal(|ui| {
                ui.label("导入");
                ui.add_space(15.0);
                if ui.text_edit_singleline(excel_path).clicked() {
                    if
                        let Some(path) = rfd::FileDialog
                            ::new()
                            .set_title("选择设备列表")
                            .pick_file()
                    {
                        self.open_excel_dialog = Some(path.display().to_string());
                    }
                }
            });

            ui.add_space(15.0);

            ui.horizontal(|ui| {
                ui.radio_value(&mut self.kind, Kind::Intranet, "内网");
                ui.radio_value(&mut self.kind, Kind::Internet, "外网");
            });

            ui.add_space(15.0);
        });

        if let Some(picked_path) = &self.open_excel_dialog {
            self.excel_path = picked_path.to_string();
            self.switchs = get_excel(self.excel_path.clone());
        }

        ui.add_space(15.0);

        TableBuilder::new(ui)
            .striped(true)
            .resizable(true)
            .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
            .column(Column::auto())
            .column(Column::auto())
            .column(Column::auto())
            .column(Column::auto())
            .column(Column::auto())
            .column(Column::auto())
            .column(Column::remainder())
            .header(20.0, |mut header| {
                header.col(|ui| {
                    ui.heading("位置");
                });
                header.col(|ui| {
                    ui.heading("设备名称");
                });
                header.col(|ui| {
                    ui.heading("设备型号");
                });
                header.col(|ui| {
                    ui.heading("厂商");
                });
                header.col(|ui| {
                    ui.heading("管理地址");
                });
                header.col(|ui| {
                    ui.heading("端口");
                });
                header.col(|ui| {
                    ui.heading("楼层");
                });
            })
            .body(|mut body| {
                let x = &self.switchs;
                let m = match self.kind {
                    Kind::Intranet => &x.0,
                    Kind::Internet => &x.1,
                };
                for switch in m {
                    body.row(30.0, |mut row| {
                        row.col(|ui| {
                            ui.label(&switch.area);
                        });
                        row.col(|ui| {
                            ui.label(&switch.name);
                        });
                        row.col(|ui| {
                            ui.label(&switch.model);
                        });
                        row.col(|ui| {
                            ui.label(&switch.factory);
                        });
                        row.col(|ui| {
                            ui.label(&switch.ip);
                        });
                        row.col(|ui| {
                            ui.label(&switch.port);
                        });
                        row.col(|ui| {
                            ui.label(&switch.floor);
                        });
                    });
                }
            });
    }
}
