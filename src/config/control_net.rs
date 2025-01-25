use crate::ui::*;
use eframe::egui;
use serde::{Deserialize, Serialize};
use std::{path::PathBuf, process::Command};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ControlNetConfig {
    pub control_net_path: PathBuf,
    pub control_net_image: PathBuf,
    pub control_stength: f32,
}
impl Default for ControlNetConfig {
    fn default() -> Self {
        Self {
            control_net_path: Default::default(),
            control_net_image: Default::default(),
            control_stength: 0.9,
        }
    }
}
impl ControlNetConfig {
    pub fn add_args<'a>(&self, command: &'a mut Command) -> &'a mut Command {
        command.args([
            "--control-net",
            &self.control_net_path.to_string_lossy(),
            "--control-image",
            &self.control_net_image.to_string_lossy(),
            "control-strength",
            &self.control_stength.to_string(),
        ])
    }
    pub fn show(&mut self, ui: &mut egui::Ui) {
        ui.collapsing("Control Net 相关", |ui| {
            model_file_select(ui, "Control Net 模型", &mut self.control_net_path);
            image_file_select(ui, ("Control Net 图像", &mut self.control_net_image));
            drag_value(
                ui,
                ("Control Net 强度", &mut self.control_stength),
                0.0..=1.0,
            ); // Please check whether the range is too small
        });
    }
}
