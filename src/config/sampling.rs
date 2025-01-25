use crate::ui::*;
use eframe::egui::{self, DragValue};
use serde::{Deserialize, Serialize};
use std::process::Command;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SamplingConfig {
    pub seed: i64,
    // 不会为 0
    pub cfg_scale: f32,
    pub slg_scale: f32,
    // 大于 0
    pub steps: u32,
    pub width: u32,
    pub height: u32,
}
impl Default for SamplingConfig {
    fn default() -> Self {
        Self {
            steps: 20,
            cfg_scale: 7.0,
            slg_scale: 0.0,
            width: 512,
            height: 512,
            seed: -1,
        }
    }
}
impl SamplingConfig {
    pub fn add_args<'a>(&self, command: &'a mut Command) -> &'a mut Command {
        command.args([
            "--seed",
            &self.seed.to_string(),
            "--width",
            &self.width.to_string(),
            "--height",
            &self.height.to_string(),
            "--steps",
            &self.steps.to_string(),
            "--cfg-scale",
            &self.cfg_scale.to_string(),
            "--slg-scale",
            &self.slg_scale.to_string(),
        ])
    }
    pub fn show(&mut self, ui: &mut egui::Ui) {
        ui.collapsing("采样参数", |ui| {
            drag_value(ui, ("种子", &mut self.seed), -1..=1145141919810);
            ui.horizontal(|ui| {
                ui.label("宽度：");
                ui.add(DragValue::new(&mut self.width).range(64..=2048).speed(64));
                ui.label("高度");
                ui.add(DragValue::new(&mut self.height).range(64..=2048).speed(64));
            });
            drag_value(ui, ("CFG Scale", &mut self.cfg_scale), 0.1..=30.0);
            drag_value(ui, ("SLG Scale", &mut self.slg_scale), 0.0..=30.0)
                .on_hover_text("仅适用于 DiT 模型（默认值：0） ");
            drag_value(ui, ("步数", &mut self.steps), 1..=150);
        });
    }
}
