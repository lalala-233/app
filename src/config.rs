use std::process::Command;

use crate::{ConvertPage, Img2ImgPage, PageType, Txt2ImgPage};
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Config {
    pub sdcpp_path: String,
    pub current_page: PageType,
    /// 模型文件路径
    pub model_path: String,
    /// VAE文件路径
    pub vae_path: String,
    /// 输出目录
    pub output_dir: String,
    /// 默认采样参数
    pub sampling: SamplingConfig,
    /// 页面配置
    pub pages: PagesConfig,

    // 新增配置项
    pub threads: i32,
    pub diffusion_model: Option<String>,
    pub clip_l_path: Option<String>,
    pub clip_g_path: Option<String>,
    pub t5xxl_path: Option<String>,
    pub taesd_path: Option<String>,
    pub control_net_path: Option<String>,
    pub embedding_dir: Option<String>,
    pub upscale_model_path: Option<String>,
    pub lora_model_dir: Option<String>,
    pub sampling_method: String,
    pub rng_type: String,
    pub batch_count: u32,
    pub schedule_type: String,
    pub clip_skip: i32,
    pub vae_tiling: bool,
    pub vae_on_cpu: bool,
    pub clip_on_cpu: bool,
    pub diffusion_fa: bool,
    pub control_net_on_cpu: bool,
    pub canny_preprocess: bool,
}

/// 采样参数配置
#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct SamplingConfig {
    pub steps: u32,
    pub cfg_scale: f32,
    pub width: u32,
    pub height: u32,
    pub seed: i64,
}

/// 页面配置
#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct PagesConfig {
    pub txt2img: Txt2ImgPage,
    pub img2img: Img2ImgPage,
    pub convert: ConvertPage,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            sdcpp_path: "./sd".to_string(),
            current_page: PageType::TextToImage,
            model_path: "model.safetensors".to_string(),
            vae_path: String::new(),
            output_dir: "output".to_string(),
            sampling: SamplingConfig {
                steps: 20,
                cfg_scale: 7.0,
                width: 512,
                height: 512,
                seed: -1,
            },
            threads: -1,
            sampling_method: "euler_a".to_string(),
            rng_type: "cuda".to_string(),
            batch_count: 1,
            schedule_type: "discrete".to_string(),
            clip_skip: -1,
            pages: Default::default(),
            diffusion_model: Default::default(),
            clip_l_path: Default::default(),
            clip_g_path: Default::default(),
            t5xxl_path: Default::default(),
            taesd_path: Default::default(),
            control_net_path: Default::default(),
            embedding_dir: Default::default(),
            upscale_model_path: Default::default(),
            lora_model_dir: Default::default(),
            vae_tiling: Default::default(),
            vae_on_cpu: Default::default(),
            clip_on_cpu: Default::default(),
            diffusion_fa: Default::default(),
            control_net_on_cpu: Default::default(),
            canny_preprocess: Default::default(),
        }
    }
}
impl Config {
    pub fn command(&self) -> Command {
        let mut command = Command::new(&self.sdcpp_path);
        command.args([
            "--model",
            &self.model_path,
            "--vae",
            &self.vae_path,
            "--seed",
            &self.sampling.seed.to_string(),
            "--width",
            &self.sampling.width.to_string(),
            "--height",
            &self.sampling.height.to_string(),
            "--steps",
            &self.sampling.steps.to_string(),
            "--cfg-scale",
            &self.sampling.cfg_scale.to_string(),
        ]);
        match self.current_page {
            PageType::TextToImage => command.args([
                "--mode",
                &PageType::TextToImage.to_string(),
                "--prompt",
                &self.pages.txt2img.prompt,
                "--negative-prompt",
                &self.pages.txt2img.negative_prompt,
                "--output",
                &self.output_dir,
            ]),
            PageType::ImageToImage => command.args([
                "--mode",
                &PageType::ImageToImage.to_string(),
                "--init-img",
                &self.pages.img2img.init_img_path,
                "--strength",
                &self.pages.img2img.strength.to_string(),
                "--output",
                &self.output_dir,
            ]),
            PageType::Convert => command.args([
                "--mode",
                &PageType::Convert.to_string(),
                "--input-img",
                &self.pages.convert.input_img_path,
                "--output",
                &self.pages.convert.convert_output_path,
            ]),
        };
        command
    }
}
