use super::AddArgs;
use crate::{ui::*, BigPathBuf};
use eframe::egui;
use serde::{Deserialize, Serialize};
use std::process::Command;
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ControlNetConfig {
    control_net_path: BigPathBuf,
    control_net_image: BigPathBuf,
    control_strength: f32,
    control_net_cpu: bool,
}
impl Default for ControlNetConfig {
    fn default() -> Self {
        Self {
            control_net_path: Default::default(),
            control_net_image: Default::default(),
            control_strength: 0.9,
            control_net_cpu: Default::default(),
        }
    }
}
impl AddArgs for ControlNetConfig {
    fn add_args(&self, command: &mut Command) {
        if self.control_net_cpu {
            command.arg("--control-net-cpu");
        }
        command.args([
            "--control-net",
            &self.control_net_path.to_string_lossy(),
            "--control-image",
            &self.control_net_image.to_string_lossy(),
            "--control-strength",
            &self.control_strength.to_string(),
        ]);
    }
}
impl ControlNetConfig {
    pub fn show(&mut self, ui: &mut egui::Ui) {
        ui.collapsing("Control Net 相关", |ui| {
            self.control_net_path.select_model(ui, "Control Net 模型");
            self.control_net_image.select_image(ui, "Control Net 图像");
            slider_value(
                ui,
                "Control Net 强度",
                &mut self.control_strength,
                0.0..=1.0,
            ); // Please tell me whether the range is too small

            ui.checkbox(&mut self.control_net_cpu, "ControlNet 在 CPU")
        });
    }
}
