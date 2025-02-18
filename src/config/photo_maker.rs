use super::AddArgs;
use crate::ui::*;
use eframe::egui;
use serde::{Deserialize, Serialize};
use std::{path::PathBuf, process::Command};
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PhotoMakerConfig {
    stacked_id_embedding_dir: PathBuf,
    input_id_images_dir: PathBuf,
    // 0..=100(%)
    style_ratio: u32,
    normalize_input: bool,
}
impl Default for PhotoMakerConfig {
    fn default() -> Self {
        Self {
            stacked_id_embedding_dir: Default::default(),
            input_id_images_dir: Default::default(),
            style_ratio: 20,
            normalize_input: Default::default(),
        }
    }
}
impl PhotoMakerConfig {
    pub fn show(&mut self, ui: &mut egui::Ui) {
        ui.collapsing("PhotoMaker 相关", |ui| {
            model_file_select(ui, "PhotoMaker 模型", &mut self.stacked_id_embedding_dir);
            folder_select(ui, "输入图片文件夹", &mut self.input_id_images_dir);
            slider_value(ui, ("风格比例", &mut self.style_ratio), 0..=100);
            ui.checkbox(&mut self.normalize_input, "归一化输入");
        });
    }
}
impl AddArgs for PhotoMakerConfig {
    fn add_args(&self, command: &mut Command) {
        if self.normalize_input {
            command.arg("--normalize-input");
        }
        command.args([
            "--stacked-id-embd-dir",
            &self.stacked_id_embedding_dir.to_string_lossy(),
            "--input-id-images-dir",
            &self.input_id_images_dir.to_string_lossy(),
            "--style-ratio",
            &self.style_ratio.to_string(),
        ]);
    }
}
