use super::AddArgs;
use eframe::egui;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct Prompts {
    pub prompt: String,
    pub negative_prompt: String,
}
impl AddArgs for Prompts {
    fn add_args(&self, command: &mut std::process::Command) {
        command.args([
            "--prompt",
            &self.prompt,
            "--negative-prompt",
            &self.negative_prompt,
        ]);
    }
}

impl Prompts {
    pub fn show(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("提示词");
            ui.text_edit_multiline(&mut self.prompt);
        });
        ui.horizontal(|ui| {
            ui.label("负面提示词");
            ui.text_edit_multiline(&mut self.negative_prompt);
        });
    }
}
