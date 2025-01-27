use super::AddArgs;
use crate::ui::*;
use eframe::egui;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EsrganConfig {
    upscale_model_path: PathBuf,
    upscale_repeats: u32,
}
impl AddArgs for EsrganConfig {
    fn add_args(&self, command: &mut std::process::Command) {
        command.args([
            "--upscale-model",
            &self.upscale_model_path.to_string_lossy(),
            "--upscale-repeats",
            &self.upscale_repeats.to_string(),
        ]);
    }
}
impl Default for EsrganConfig {
    fn default() -> Self {
        Self {
            upscale_model_path: Default::default(),
            upscale_repeats: 1,
        }
    }
}
impl EsrganConfig {
    pub fn show(&mut self, ui: &mut egui::Ui) {
        ui.collapsing("Upscale", |ui| {
            model_file_select(ui, "ESRGAN 模型", &mut self.upscale_model_path)
                .on_hover_text("仅支持 RealESRGAN_x4plus_anime_6B");
            slider_value(ui, ("超分辨率次数", &mut self.upscale_repeats), 1..=16);
        });
    }
}
