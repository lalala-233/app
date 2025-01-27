mod control_net;
mod flags;
mod pages;
mod photo_maker;
mod prompt;
mod rng;
mod sampling;
mod sampling_method;
mod schedule;
mod skip;
mod weight_type;
use crate::ui::*;
use control_net::ControlNetConfig;
use eframe::egui::Ui;
use flags::Flags;
use pages::PagesConfig;
use photo_maker::PhotoMakerConfig;
use prompt::Prompts;
use rng::RngType;
use sampling::SamplingConfig;
use sampling_method::SamplingMethod;
use schedule::Schedule;
use serde::{Deserialize, Serialize};
use skip::SkipConfig;
use std::path::PathBuf;
use std::process::Command;
use weight_type::WeightType;
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Configs {
    pub sdcpp_path: PathBuf,
    pub pages_config: PagesConfig,
    pub control_net_config: ControlNetConfig,
    pub photo_maker_config: PhotoMakerConfig,
    pub sampling_config: SamplingConfig,
    pub weight_type: WeightType,
    pub rng_type: RngType,
    pub sampling_method: SamplingMethod,
    pub schedule_type: Schedule,
    pub flags: Flags,
    pub skip_config: SkipConfig,
    pub prompts: Prompts,
    pub threads: i32,
    pub model_path: PathBuf,
    pub _diffusion_model: PathBuf, // sdcpp 中支持单独指定 diffusion_model 并外接 VAE 等，不知道直接使用 model 指定是否可以外接，暂不实现
    pub clip_l_path: PathBuf,
    pub clip_g_path: PathBuf,
    pub t5xxl_path: PathBuf,
    pub vae_path: PathBuf,
    pub taesd_path: PathBuf,
    pub embedding_dir: PathBuf,
    pub upscale_model_path: PathBuf,
    pub upscale_repeats: u32,
    pub lora_model_dir: PathBuf,
    pub batch_count: u32,
    pub output_path: PathBuf,
}

impl Default for Configs {
    fn default() -> Self {
        Self {
            _diffusion_model: Default::default(),
            clip_l_path: Default::default(),
            clip_g_path: Default::default(),
            t5xxl_path: Default::default(),
            vae_path: Default::default(),
            taesd_path: Default::default(),
            control_net_config: Default::default(),
            embedding_dir: Default::default(),
            upscale_model_path: Default::default(),
            weight_type: Default::default(),
            lora_model_dir: Default::default(),
            sampling_config: Default::default(),
            sampling_method: Default::default(),
            rng_type: Default::default(),
            schedule_type: Default::default(),
            pages_config: Default::default(),
            flags: Default::default(),
            skip_config: Default::default(),
            photo_maker_config: Default::default(),
            sdcpp_path: Default::default(),
            model_path: Default::default(),
            output_path: Default::default(),
            prompts: Default::default(),
            threads: -1,
            upscale_repeats: 1,
            batch_count: 1,
        }
    }
}

impl Configs {
    pub fn show(&mut self, ui: &mut Ui) {
        file_select(
            ui,
            true,
            ("sdcpp 路径", &mut self.sdcpp_path),
            Default::default(),
        );
        model_file_select(ui, "模型", &mut self.model_path);
        self.pages_config.show(ui);
    }
    fn get_add_args(&self) -> impl Iterator<Item = &dyn AddArgs> {
        [
            self as &dyn AddArgs,
            &self.prompts,
            &self.pages_config,
            &self.control_net_config,
            &self.photo_maker_config,
            &self.sampling_config,
            &self.weight_type,
            &self.rng_type,
            &self.sampling_method,
            &self.schedule_type,
            &self.flags,
            &self.skip_config,
        ]
        .into_iter()
    }
    pub fn command(&self) -> Command {
        let mut command = Command::new(&self.sdcpp_path);
        let configs = self.get_add_args();
        for config in configs {
            config.add_args(&mut command);
        }
        command
    }
}
impl AddArgs for Configs {
    fn add_args(&self, command: &mut Command) {
        command.args([
            "--threads",
            &self.threads.to_string(),
            "--model",
            &self.model_path.to_string_lossy(),
            "--lora-model-dir",
            &self.lora_model_dir.to_string_lossy(),
            "--clip_l",
            &self.clip_l_path.to_string_lossy(),
            "--clip_g",
            &self.clip_g_path.to_string_lossy(),
            "--t5xxl",
            &self.t5xxl_path.to_string_lossy(),
            "--vae",
            &self.vae_path.to_string_lossy(),
            "--taesd",
            &self.taesd_path.to_string_lossy(),
            "--embd-dir",
            &self.embedding_dir.to_string_lossy(),
            "--upscale-model",
            &self.upscale_model_path.to_string_lossy(),
            "--upscale-repeats",
            &self.upscale_repeats.to_string(),
            "--batch-count",
            &self.batch_count.to_string(),
            "--output",
            &self.output_path.to_string_lossy(),
        ]);
    }
}

trait AddArgs {
    fn add_args(&self, command: &mut Command);
}
