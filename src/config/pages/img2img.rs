use crate::{config::AddArgs, ui::*};
use eframe::egui;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct Img2ImgPage {
    init_img_path: PathBuf,
    mask_img_path: PathBuf,
    // 大于 0，但不知道最大值是多少，故限制为 10.0
    guidance: f32,
    strength: f32,
}
impl AddArgs for Img2ImgPage {
    fn add_args(&self, command: &mut std::process::Command) {
        command.args([
            "--init-img",
            &self.init_img_path.to_string_lossy(),
            "--mask",
            &self.mask_img_path.to_string_lossy(),
            "--guidance",
            &self.guidance.to_string(),
            "--strength",
            &self.strength.to_string(),
        ]);
    }
}
impl Img2ImgPage {
    pub fn show(&mut self, ui: &mut egui::Ui) {
        ui.heading("图生图");
        image_file_select(ui, "输入图片", &mut self.init_img_path);
        image_file_select(ui, "Mask 图片", &mut self.mask_img_path)
            .on_hover_text("需要使用 Inpaint 模型");
        slider_value(ui, ("guidance", &mut self.guidance), 0.0..=10.0);
        slider_value(ui, ("强度", &mut self.strength), 0.0..=1.0);
    }
}
