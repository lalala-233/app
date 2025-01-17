use eframe::egui;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct ConvertPage {
    pub input_img_path: PathBuf,
}

impl ConvertPage {
    pub fn show(&mut self, ui: &mut egui::Ui) {
        ui.heading("格式转换");
    }
}
