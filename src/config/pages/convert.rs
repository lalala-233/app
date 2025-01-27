use eframe::egui;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct ConvertPage {}
impl ConvertPage {
    pub fn show(&mut self, ui: &mut egui::Ui) {
        ui.heading("格式转换");
    }
}
