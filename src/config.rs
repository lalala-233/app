mod control_net;
mod flags;
mod rng;
mod sampling;
mod sampling_method;
mod schedule;
mod skip;
mod weight_type;

use crate::{ConvertPage, Img2ImgPage, PageType, Txt2ImgPage};
use control_net::ControlNetConfig;
use flags::Flags;
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
pub struct Config {
    pub sdcpp_path: PathBuf,
    pub current_page: PageType,
    pub pages: PagesConfig,
    pub threads: i32,
    pub model_path: PathBuf,
    pub _diffusion_model: PathBuf, // sdcpp 中支持单独指定 diffusion_model 并外接 VAE 等，不知道直接使用 model 指定是否可以外接，暂不实现
    pub clip_l_path: PathBuf,
    pub clip_g_path: PathBuf,
    pub t5xxl_path: PathBuf,
    pub vae_path: PathBuf,
    pub taesd_path: PathBuf,
    pub control_net_config: ControlNetConfig,
    pub embedding_dir: PathBuf,
    pub stacked_id_embedding_dir: PathBuf,
    pub input_id_images_dir: PathBuf,
    pub sampling_config: SamplingConfig,
    pub upscale_model_path: PathBuf,
    pub upscale_repeats: u32,
    pub weight_type: WeightType,
    pub lora_model_dir: PathBuf,
    pub sampling_method: SamplingMethod,
    pub rng_type: RngType,
    pub batch_count: u32,
    pub schedule_type: Schedule,
    pub flags: Flags,
    pub skip_config: SkipConfig,
    pub output_path: PathBuf,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            current_page: Default::default(),
            _diffusion_model: Default::default(),
            clip_l_path: Default::default(),
            clip_g_path: Default::default(),
            t5xxl_path: Default::default(),
            vae_path: PathBuf::new(),
            taesd_path: Default::default(),
            control_net_config: Default::default(),
            embedding_dir: Default::default(),
            stacked_id_embedding_dir: Default::default(),
            input_id_images_dir: Default::default(),
            upscale_model_path: Default::default(),
            weight_type: Default::default(),
            lora_model_dir: Default::default(),
            sampling_config: Default::default(),
            sampling_method: Default::default(),
            rng_type: Default::default(),
            schedule_type: Default::default(),
            pages: Default::default(),
            flags: Default::default(),
            skip_config: Default::default(),
            sdcpp_path: PathBuf::from("./sd"),
            model_path: PathBuf::from("model.safetensors"),
            output_path: PathBuf::from("output"),
            threads: -1,
            upscale_repeats: 1,
            batch_count: 1,
        }
    }
}

/// 页面配置
#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct PagesConfig {
    pub txt2img: Txt2ImgPage,
    pub img2img: Img2ImgPage,
    pub convert: ConvertPage,
}

impl Config {
    pub fn command(&self) -> Command {
        let mut command = Command::new(&self.sdcpp_path);
        self.add_args(&mut command);
        self.control_net_config.add_args(&mut command);
        self.skip_config.add_args(&mut command);
        self.sampling_config.add_args(&mut command);
        self.flags.add_flags(&mut command);
        match self.current_page {
            PageType::Txt2Img => command.args([
                "--mode",
                PageType::Txt2Img.as_ref(),
                "--prompt",
                &self.pages.txt2img.prompt,
                "--negative-prompt",
                &self.pages.txt2img.negative_prompt,
            ]),
            PageType::Img2Img => command.args([
                "--mode",
                PageType::Img2Img.as_ref(),
                "--init-img",
                &self.pages.img2img.init_img_path.to_string_lossy(),
                "--mask",
                &self.pages.img2img.mask_img_path.to_string_lossy(),
                "--guidance",
                &self.pages.img2img.guidance.to_string(),
                "--strength",
                &self.pages.img2img.strength.to_string(),
            ]),
            PageType::Convert => command.args([
                "--mode",
                PageType::Convert.as_ref(),
                "--input-img",
                &self.pages.convert.input_img_path.to_string_lossy(),
            ]),
        };
        command
    }
    fn add_args<'a>(&self, command: &'a mut Command) -> &'a mut Command {
        command.args([
            "--threads",
            &self.threads.to_string(),
            "--model",
            &self.model_path.to_string_lossy(),
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
            "--stacked-id-embd-dir",
            &self.stacked_id_embedding_dir.to_string_lossy(),
            "--input-id-images-dir",
            &self.input_id_images_dir.to_string_lossy(),
            "--upscale-model",
            &self.upscale_model_path.to_string_lossy(),
            "--upscale-repeats",
            &self.upscale_repeats.to_string(),
            "--sampling-method",
            self.sampling_method.as_ref(),
            "--rng",
            self.rng_type.as_ref(),
            "--batch-count",
            &self.batch_count.to_string(),
            "--schedule",
            self.schedule_type.as_ref(),
            "--output",
            &self.output_path.to_string_lossy(),
        ])
    }
}
