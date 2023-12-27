use chrono::{DateTime, Local};
use crossbeam::channel::{Receiver, Sender};
use eframe::egui::{self, Context, ProgressBar, Ui, Window};
use eframe::epaint::Color32;
use std::fs;
use std::sync::atomic::Ordering::{self, Release};

use crate::backup::{NUM_DONE, STOP};
use crate::{
    backup::handle,
    model::{blj::Kind, switch::Switch},
    setting::Setting,
};

pub struct Backup {
    kind: Kind,
    data: (Vec<Switch>, Vec<Switch>),
    setting: Setting,
    tx: Sender<f64>,
    rx: Receiver<f64>,
    count: f32,
    animate_progress_bar: bool,
    visible: bool,
    internet_failed: Vec<String>,
    intranet_failed: Vec<String>,
    tx_failed: Sender<(Kind, String)>,
    rx_failed: Receiver<(Kind, String)>,
    start: Start,
}

enum Start {
    Yes,
    No,
}

type FailChannel = (Sender<(Kind, String)>, Receiver<(Kind, String)>);

impl Default for Backup {
    fn default() -> Self {
        let (tx, rx): (Sender<f64>, Receiver<f64>) = crossbeam::channel::unbounded();
        let (tx_failed, rx_failed): FailChannel = crossbeam::channel::unbounded();

        Self {
            kind: Kind::Intranet,
            data: Switch::global().clone(),
            setting: Setting::new(),
            tx,
            rx,
            count: 0.0,
            animate_progress_bar: false,
            visible: false,
            internet_failed: Vec::new(),
            intranet_failed: Vec::new(),
            tx_failed,
            rx_failed,
            start: Start::No,
        }
    }
}

impl Backup {
    pub fn name(&self) -> &'static str {
        "🖴 备份"
    }

    pub fn show(&mut self, ctx: &Context, open: &mut bool) {
        Window::new(self.name())
            .default_pos([800.0, 0.0])
            .default_width(500.0)
            .vscroll(true)
            .hscroll(true)
            .open(open)
            .show(ctx, |ui| self.ui(ui));
    }

    pub fn ui(&mut self, ui: &mut Ui) {
        let data = match self.kind {
            Kind::Intranet => &self.data.0,
            Kind::Internet => &self.data.1,
        };

        let mut backuped = false;
        let dt: DateTime<Local> = Local::now();
        let dir = dt.format("%Y_%m_%d").to_string();
        if let Ok(rd) = fs::read_dir("./output") {
            for res in rd {
                let entry = res.unwrap();
                if entry.file_name().into_string().unwrap() == dir {
                    backuped = true;
                    break;
                }
            }
        }
        ui.scope(|ui| {
            if backuped {
                ui.label("今天已经备份");
            } else {
                ui.visuals_mut().override_text_color = Some(Color32::RED.linear_multiply(0.5));
                ui.label("今天还没有备份");
            }
        });

        ui.add_space(15.0);
        ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
            ui.radio_value(&mut self.kind, Kind::Intranet, "内网");
            ui.radio_value(&mut self.kind, Kind::Internet, "外网");
        });

        ui.add_space(15.0);
        ui.horizontal(|ui| {
            if ui.button("开始").clicked() {
                self.start = Start::Yes;
                self.intranet_failed.clear();
                self.internet_failed.clear();
                self.count = 0.0;
                self.visible = true;

                STOP.store(false, Release);
                NUM_DONE.store(0, Release);

                handle(
                    data.clone(),
                    self.setting.clone(),
                    self.tx.clone(),
                    self.tx_failed.clone(),
                    self.kind,
                )
                .unwrap();
            }

            if ui.button("停止").clicked() {
                STOP.store(true, Ordering::Relaxed);
                dbg!(STOP.load(Ordering::Relaxed));
            }

            if ui.button("清除").clicked() {
                self.intranet_failed.clear();
                self.internet_failed.clear();
                self.count = 0.0;
                self.visible = false;
                self.start = Start::No;
            }

            ui.add_space(12.0);
            ui.set_visible(self.visible);

            if let Ok(count) = self.rx.try_recv() {
                self.count = count as f32;
            }

            let progress_bar = ProgressBar::new(self.count)
                .show_percentage()
                .animate(self.animate_progress_bar)
                .desired_width(250.0);

            ui.add(progress_bar);

            if self.count >= 0.0 {
                self.animate_progress_bar = true;
            }

            if self.count == 1.0 {
                self.visible = false;
                self.animate_progress_bar = false;
            }
        });

        ui.add_space(15.0);
        ui.label("结果:");
        ui.separator();

        if let Ok(fail) = self.rx_failed.try_recv() {
            match fail {
                (Kind::Internet, s) => self.internet_failed.push(s),
                (Kind::Intranet, s) => self.intranet_failed.push(s),
            }
        }

        let total = data.len();

        let internet_failed = self.internet_failed.len();
        let intranet_failed = self.intranet_failed.len();

        let mut success;
        let fail;
        let failed;

        if let Kind::Internet = self.kind {
            success = total.saturating_sub(internet_failed);
            fail = internet_failed;
            failed = &self.internet_failed;
        } else {
            success = total.saturating_sub(intranet_failed);
            fail = intranet_failed;
            failed = &self.intranet_failed;
        }

        if let Start::No = self.start {
            success = 0;
        }

        let success_count = format!("成功: {}", success);
        let failed_count = format!("失败: {}", fail);

        ui.horizontal(|ui| {
            ui.label(format!("总数: {}", total));
            ui.label(success_count);
            ui.label(failed_count);
        });

        ui.add_space(15.0);
        for fail in failed.iter() {
            ui.push_id(fail, |ui| {
                ui.label(fail);
            });
        }
    }
}
