use crate::ui::*;
use eframe::egui;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct Img2ImgPage {
    pub init_img_path: PathBuf,
    pub mask_img_path: PathBuf,
    // 大于 0，但不知道最大值是多少
    pub guidance: f32,
    pub strength: f32,
}

impl Img2ImgPage {
    pub fn show(&mut self, ui: &mut egui::Ui) {
        ui.heading("图生图");
        file_select(
            ui,
            true,
            ("输入图片：", &mut self.init_img_path),
            ("图片文件", &["png", "jpg", "jpeg", "bmp"]),
        );
        drag_value(ui,"guidance：",&mut self.strength,0.0..=10.0);
        drag_value(ui,"强度：",&mut self.strength,0.0..=1.0);
    }
}
