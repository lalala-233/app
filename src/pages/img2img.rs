use eframe::egui;

use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Default,Clone,Debug)]
pub struct Img2ImgPage {
    pub init_img_path: String,
    pub strength: f32,
}

impl Img2ImgPage {
    pub fn show(&mut self, ui: &mut egui::Ui) {
        ui.heading("图生图");
        ui.horizontal(|ui| {
            ui.label("输入图片:");
            ui.text_edit_singleline(&mut self.init_img_path);
            if ui.button("选择...").clicked() {
                if let Some(path) = rfd::FileDialog::new()
                    .add_filter("图片文件", &["png", "jpg", "jpeg"])
                    .pick_file()
                {
                    self.init_img_path = path.to_string_lossy().to_string();
                }
            }
        });
        ui.horizontal(|ui| {
            ui.label("强度:");
            ui.add(egui::DragValue::new(&mut self.strength).range(0.0..=1.0));
        });
    }
}
