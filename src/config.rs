use crate::{ConvertPage, Img2ImgPage, PageType, Txt2ImgPage};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::process::Command;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Config {
    pub sdcpp_path: PathBuf,
    pub current_page: PageType,
    pub model_path: PathBuf,
    pub vae_path: PathBuf,
    pub output_dir: PathBuf,
    pub sampling: SamplingConfig,
    pub pages: PagesConfig,
    pub threads: i32,
    pub diffusion_model: Option<PathBuf>,
    pub clip_l_path: Option<PathBuf>,
    pub clip_g_path: Option<PathBuf>,
    pub t5xxl_path: Option<PathBuf>,
    pub taesd_path: Option<PathBuf>,
    pub control_net_path: Option<PathBuf>,
    pub embedding_dir: Option<PathBuf>,
    pub upscale_model_path: Option<PathBuf>,
    pub lora_model_dir: Option<PathBuf>,
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

impl Default for Config {
    fn default() -> Self {
        Self {
            sdcpp_path: PathBuf::from("./sd"),
            current_page: PageType::TextToImage,
            model_path: PathBuf::from("model.safetensors"),
            vae_path: PathBuf::new(),
            output_dir: PathBuf::from("output"),
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

/// 采样参数配置
#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct SamplingConfig {
    pub seed: i64,
    // 不会为 0
    pub cfg_scale: f32,
    pub steps: u32,
    pub width: u32,
    pub height: u32,
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
        command.args([
            "--model",
            self.model_path.to_str().unwrap(),
            "--vae",
            self.vae_path.to_str().unwrap(),
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
            "--threads",
            &self.threads.to_string(),
            "--sampling-method",
            &self.sampling_method,
            "--rng",
            &self.rng_type,
            "--batch-count",
            &self.batch_count.to_string(),
            "--schedule",
            &self.schedule_type,
            "--clip-skip",
            &self.clip_skip.to_string(),
            "--output",
            self.output_dir.to_str().unwrap(),
        ]);
        match self.current_page {
            PageType::TextToImage => command.args([
                "--mode",
                &PageType::TextToImage.to_string(),
                "--prompt",
                &self.pages.txt2img.prompt,
                "--negative-prompt",
                &self.pages.txt2img.negative_prompt,
            ]),
            PageType::ImageToImage => command.args([
                "--mode",
                &PageType::ImageToImage.to_string(),
                "--init-img",
                self.pages.img2img.init_img_path.to_str().unwrap(),
                "--strength",
                &self.pages.img2img.strength.to_string(),
            ]),
            PageType::Convert => command.args([
                "--mode",
                &PageType::Convert.to_string(),
                "--input-img",
                self.pages.convert.input_img_path.to_str().unwrap(),
            ]),
        };
        command
    }
}
