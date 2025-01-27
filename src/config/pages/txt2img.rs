use eframe::egui;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct Txt2ImgPage {}

impl Txt2ImgPage {
    pub fn show(&mut self, ui: &mut egui::Ui) {
        ui.heading("文生图");
    }
}
