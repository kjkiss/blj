use eframe::egui;

pub mod backup;
pub mod setting;
pub mod command;

pub trait View {
    fn ui(&mut self, ui: &mut egui::Ui);
}

pub trait Window {
    fn name(&self) -> &'static str;

    fn show(&mut self, ctx: &egui::Context, open: &mut bool);
}
