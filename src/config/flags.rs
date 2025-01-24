use serde::{Deserialize, Serialize};
use std::process::Command;
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Flags {
    pub normalize_input: bool,
    pub vae_tiling: bool,
    pub vae_on_cpu: bool,
    pub clip_on_cpu: bool,
    pub diffusion_fa: bool,
    pub control_net_cpu: bool,
    pub canny: bool,
}
impl Flags {
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&mut bool, &str)> {
        [
            (&mut self.normalize_input, "标准化 PhotoMaker 输入图片"),
            (&mut self.vae_tiling, "VAE 分块处理"),
            (&mut self.vae_on_cpu, "VAE 在 CPU"),
            (&mut self.clip_on_cpu, "CLIP 在 CPU"),
            (&mut self.diffusion_fa, "扩散模型 flash attention"),
            (&mut self.control_net_cpu, "ControlNet 在 CPU"),
            (&mut self.canny, "Canny 预处理"),
        ]
        .into_iter()
    }
    pub fn add_flags<'a>(&self, command: &'a mut Command) -> &'a mut Command {
        if self.normalize_input {
            command.arg("--normalize-input");
        }
        if self.vae_tiling {
            command.arg("--vae-tiling");
        }
        if self.vae_on_cpu {
            command.arg("--vae-on-cpu");
        }
        if self.clip_on_cpu {
            command.arg("--clip-on-cpu");
        }
        if self.diffusion_fa {
            command.arg("--diffusion-fa");
        }
        if self.control_net_cpu {
            command.arg("--control-net-cpu");
        }
        if self.canny {
            command.arg("--canny");
        }
        command
    }
}
