use crate::ui::file_select_config;
use eframe::egui;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct Img2ImgPage {
    pub init_img_path: PathBuf,
    pub strength: f32,
}

impl Img2ImgPage {
    pub fn show(&mut self, ui: &mut egui::Ui) {
        ui.heading("图生图");
        file_select_config(
            ui,
            ("输入图片：", &mut self.init_img_path),
            ("图片文件", &["png", "jpg", "jpeg"]),
        );
        ui.horizontal(|ui| {
            ui.label("强度：");
            ui.add(egui::DragValue::new(&mut self.strength).range(0.0..=1.0));
        });
    }
}
