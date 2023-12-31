#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::model::blj::{ Area, Blj, Kind };
use crate::model::store::Store;
use eframe::egui::Style;
use eframe::{ egui::{ self, Context, Layout, TextStyle }, emath::Align, Frame };
use rusqlite::Connection;

mod backup;
mod font;
mod model;
mod row;
mod setting;
mod windows;
mod write;

fn main() -> Result<(), eframe::Error> {
    {
        let conn = Connection::open("blj.db").unwrap();
        conn.execute(
            "CREATE TABLE IF NOT EXISTS store (
                id INTEGER PRIMARY KEY,
                username TEXT,
                password TEXT,
                crtpath TEXT,
                excelpath TEXT
            )",
            ()
        ).unwrap();

        conn.execute(
            "CREATE TABLE IF NOT EXISTS commands (
                factory TEXT PRIMARY KEY,
                command TEXT
            )",
            ()
        ).unwrap();
    }

    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Mini堡垒机",
        native_options,
        Box::new(|cc| Box::new(Blj::new(cc)))
    )
}

impl Blj {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        font::setup_custom_fonts(&cc.egui_ctx);
        Self::default()
    }
}

impl eframe::App for Blj {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        // 左边区域面板
        egui::SidePanel::left("my_left_panel").show(ctx, |ui| {
            ui.add_space(10.0);
            ui.scope(|ui| {
                ui.style_mut().override_text_style = Some(TextStyle::Body);
                ui.label("区域");
            });
            ui.add_space(20.0);
            ui.radio_value(&mut self.area, Area::Ytbz, "月坛北座");
            ui.radio_value(&mut self.area, Area::Ytnz, "月坛南座");
            ui.radio_value(&mut self.area, Area::FT, "丰台");
            ui.radio_value(&mut self.area, Area::Yyr, "远洋锐");
            ui.radio_value(&mut self.area, Area::CP, "昌平");
            ui.radio_value(&mut self.area, Area::Xhl, "新华里");
        });

        // 顶部选项面板
        egui::TopBottomPanel
            ::top("my_top_panel")
            .default_height(40.0)
            .show(ctx, |ui| {
                ui.horizontal_centered(|ui| {
                    ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                        ui.radio_value(&mut self.kind, Kind::Intranet, "内网");
                        ui.radio_value(&mut self.kind, Kind::Internet, "外网");
                    });
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        egui::widgets::global_dark_light_mode_switch(ui);
                        ui.separator();
                        ui.toggle_value(&mut self.backup_is_open, self.backup.name());
                        self.checkboxes(ui);
                    });
                });
            });
        // 中间展示面板
        egui::CentralPanel::default().show(ctx, |ui| {
            let area = match self.area {
                Area::Ytbz => "月坛北座",
                Area::Ytnz => "月坛南座",
                Area::FT => "丰台",
                Area::Yyr => "远洋锐",
                Area::CP => "昌平",
                Area::Xhl => "新华里",
            };

            let switchs = match self.kind {
                Kind::Internet => self.data.1.clone(),
                Kind::Intranet => self.data.0.clone(),
            };

            ui.heading(area);
            ui.add_space(10.0);

            let mut style: Style = (*ui.ctx().style()).clone();
            style.spacing.scroll.bar_width = 6.0; // 宽度
            style.spacing.scroll.floating_allocated_width = 10.0; // 与内容的距离
            style.spacing.scroll.interact_handle_opacity = 0.5; // 交互时的透明度
            ui.set_style(style);

            egui::ScrollArea::vertical().show(ui, |ui| {
                row::row_floor(switchs, area, self.store.clone(), ui);
                ui.allocate_space(egui::vec2(0.0, 100.0));
            });
        });

        self.backup.show(ctx, &mut self.backup_is_open);
        self.windows(ctx);
    }
}
