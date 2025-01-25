use serde::{Deserialize, Serialize};
use std::process::Command;
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Flags {
    vae_tiling: bool,
    vae_on_cpu: bool,
    clip_on_cpu: bool,
    diffusion_fa: bool,
    canny: bool,
}
impl Flags {
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&mut bool, &str)> {
        [
            (&mut self.vae_tiling, "VAE 分块处理"),
            (&mut self.vae_on_cpu, "VAE 在 CPU"),
            (&mut self.clip_on_cpu, "CLIP 在 CPU"),
            (&mut self.diffusion_fa, "扩散模型 flash attention"),
            (&mut self.canny, "Canny 预处理"),
        ]
        .into_iter()
    }
    pub fn add_flags(&self, command: &mut Command)  {
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
        if self.canny {
            command.arg("--canny");
        }
    }
}
