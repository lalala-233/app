// because I don't know the skip parameter, so there may be something wrong.
use crate::ui::*;
use eframe::egui;
use serde::{Deserialize, Serialize};
use std::process::Command;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SkipConfig {
    clip_skip: i32,
    skip_layers: String,
    skip_layer_start: f32,
    skip_layer_end: f32,
}
impl Default for SkipConfig {
    fn default() -> Self {
        Self {
            clip_skip: -1, // 1 ignores none, 2 ignores one layer
            skip_layers: "[7, 8, 9]".to_string(),
            skip_layer_start: 0.01,
            skip_layer_end: 0.20,
        }
    }
}
impl SkipConfig {
    pub fn add_args(&self, command: &mut Command) {
        command.args([
            "--clip-skip",
            &self.clip_skip.to_string(),
            "--skip-layers",
            &self.skip_layers,
            "--skip-layer-start",
            &self.skip_layer_start.to_string(),
            "--skip-layer-end",
            &self.skip_layer_end.to_string(),
        ]);
    }
    pub fn show(&mut self, ui: &mut egui::Ui) {
        ui.collapsing("Skip 相关", |ui| {
            slider_value(ui, ("clip-skip", &mut self.clip_skip), -1..=114)
                .on_hover_text("1 ignores none, 2 ignores one layer");
            ui.horizontal(|ui| {
                ui.label("skip-layers");
                ui.text_edit_singleline(&mut self.skip_layers);
            });
            slider_value(
                ui,
                ("clip-layer-start", &mut self.skip_layer_start),
                0.01..=1.00,
            );
            slider_value(ui, ("clip-skip", &mut self.skip_layer_end), 0.01..=1.00);
        });
    }
}
