use crate::PageType;
use std::process::Command;
pub struct CommandBuilder {
    command: Command,
}

impl CommandBuilder {
    pub fn new(program: &str) -> Self {
        Self {
            command: Command::new(program),
        }
    }

    pub fn model(mut self, model_path: &str) -> Self {
        self.command.arg("--model").arg(model_path);
        self
    }

    pub fn vae(mut self, vae_path: &str) -> Self {
        self.command.arg("--vae").arg(vae_path);
        self
    }

    pub fn seed(mut self, seed: i64) -> Self {
        self.command.arg("--seed").arg(seed.to_string());
        self
    }

    pub fn width(mut self, width: u32) -> Self {
        self.command.arg("--width").arg(width.to_string());
        self
    }

    pub fn height(mut self, height: u32) -> Self {
        self.command.arg("--height").arg(height.to_string());
        self
    }

    pub fn steps(mut self, steps: u32) -> Self {
        self.command.arg("--steps").arg(steps.to_string());
        self
    }

    pub fn cfg_scale(mut self, cfg_scale: f32) -> Self {
        self.command.arg("--cfg-scale").arg(cfg_scale.to_string());
        self
    }

    pub fn mode(mut self, mode: PageType) -> Self {
        self.command.arg("--mode").arg(mode.to_string());
        self
    }

    pub fn prompt(mut self, prompt: &str) -> Self {
        self.command.arg("--prompt").arg(prompt);
        self
    }

    pub fn negative_prompt(mut self, negative_prompt: &str) -> Self {
        self.command.arg("--negative-prompt").arg(negative_prompt);
        self
    }

    pub fn init_img(mut self, init_img_path: &str) -> Self {
        self.command.arg("--init-img").arg(init_img_path);
        self
    }

    pub fn strength(mut self, strength: f32) -> Self {
        self.command.arg("--strength").arg(strength.to_string());
        self
    }

    pub fn input_img(mut self, input_img_path: &str) -> Self {
        self.command.arg("--input-img").arg(input_img_path);
        self
    }

    pub fn output(mut self, output_path: &str) -> Self {
        self.command.arg("--output").arg(output_path);
        self
    }

    pub fn build(self) -> Command {
        self.command
    }
}
