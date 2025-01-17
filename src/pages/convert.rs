use eframe::egui;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct ConvertPage {
    pub input_img_path: String,
    pub convert_output_path: String,
}

impl ConvertPage {
    pub fn show(&mut self, ui: &mut egui::Ui) {
        ui.heading("格式转换");
        ui.horizontal(|ui| {
            ui.label("输入图片:");
            ui.text_edit_singleline(&mut self.input_img_path);
            if ui.button("选择...").clicked() {
                if let Some(path) = rfd::FileDialog::new()
                    .add_filter("图片文件", &["png", "jpg", "jpeg"])
                    .pick_file()
                {
                    self.input_img_path = path.to_string_lossy().to_string();
                }
            }
        });
        ui.horizontal(|ui| {
            ui.label("输出路径:");
            ui.text_edit_singleline(&mut self.convert_output_path);
            if ui.button("选择...").clicked() {
                if let Some(path) = rfd::FileDialog::new()
                    .add_filter("图片文件", &["png", "jpg", "jpeg"])
                    .save_file()
                {
                    self.convert_output_path = path.to_string_lossy().to_string();
                }
            }
        });
    }
}
