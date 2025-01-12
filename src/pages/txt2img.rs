use eframe::egui;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct Txt2ImgPage {
    pub prompt: String,
    pub negative_prompt: String,
}

impl Txt2ImgPage {
    pub fn show(&mut self, ui: &mut egui::Ui) {
        ui.heading("文生图");
        ui.horizontal(|ui| {
            ui.label("提示词:");
            ui.text_edit_multiline(&mut self.prompt);
        });
        ui.horizontal(|ui| {
            ui.label("负面提示词:");
            ui.text_edit_multiline(&mut self.negative_prompt);
        });
    }
}
