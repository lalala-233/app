use super::AddArgs;
use crate::ui::*;
use eframe::egui::{self, Color32, DragValue};
use serde::{Deserialize, Serialize};
use std::process::Command;
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SamplingConfig {
    seed: String,
    // 不会为 0
    cfg_scale: f32,
    slg_scale: f32,
    // 大于 0
    steps: u32,
    width: u32,
    height: u32,
}
impl Default for SamplingConfig {
    fn default() -> Self {
        Self {
            steps: 20,
            cfg_scale: 7.0,
            slg_scale: 0.0,
            width: 512,
            height: 512,
            seed: "-1".to_string(),
        }
    }
}
impl AddArgs for SamplingConfig {
    fn add_args(&self, command: &mut Command) {
        command.args([
            "--seed",
            &self.seed,
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
        ]);
    }
}
impl SamplingConfig {
    pub fn show(&mut self, ui: &mut egui::Ui) {
        ui.collapsing("采样参数", |ui| {
            ui.horizontal(|ui| {
                ui.label("种子");
                ui.text_edit_singleline(&mut self.seed);
                if self.seed.parse::<i64>().is_err() {
                    ui.colored_label(Color32::RED, format!("请输入 -1~{} 的整数", i64::MAX));
                }
            });
            ui.horizontal(|ui| {
                ui.label("宽度：");
                ui.add(DragValue::new(&mut self.width).range(64..=2048).speed(64));
                ui.label("高度");
                ui.add(DragValue::new(&mut self.height).range(64..=2048).speed(64));
            });
            slider_value(ui, "CFG Scale", &mut self.cfg_scale, 0.1..=30.0);
            slider_value(ui, "SLG Scale", &mut self.slg_scale, 0.0..=30.0)
                .on_hover_text("仅适用于 DiT 模型（默认值：0） ");
            slider_value(ui, "步数", &mut self.steps, 1..=150);
        });
    }
}
