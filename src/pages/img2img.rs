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
        image_file_select(ui, ("输入图片", &mut self.init_img_path));
        image_file_select(ui, ("Mask 图片", &mut self.mask_img_path))
            .on_hover_text("需要使用 Inpaint 模型");
        drag_value(ui, ("guidance", &mut self.strength), 0.0..=10.0);
        drag_value(ui, ("强度", &mut self.strength), 0.0..=1.0);
    }
}
