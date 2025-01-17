use crate::ui::*;
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
        file_select_config(
            ui,
            ("输入图片：", &mut self.input_img_path),
            ("图片文件", &["png", "jpg", "jpeg"]),
        );
    }
}
