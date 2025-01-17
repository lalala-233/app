use crate::{ConvertPage, Img2ImgPage, PageType, Txt2ImgPage};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::process::Command;

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
    pub control_net_path: PathBuf,
    pub embedding_dir: PathBuf,
    pub stacked_id_embedding_dir: PathBuf,
    pub input_id_images_dir: PathBuf,
    pub normalize_input: bool,
    pub sampling: SamplingConfig,
    pub upscale_model_path: PathBuf,
    pub upscale_repeats: u32,
    pub weight_type: WeightType,
    pub lora_model_dir: PathBuf,
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
    pub output_path: PathBuf,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            sdcpp_path: PathBuf::from("./sd"),
            current_page: PageType::TextToImage,
            threads: -1,
            model_path: PathBuf::from("model.safetensors"),
            _diffusion_model: Default::default(),
            clip_l_path: Default::default(),
            clip_g_path: Default::default(),
            t5xxl_path: Default::default(),
            vae_path: PathBuf::new(),
            taesd_path: Default::default(),
            control_net_path: Default::default(),
            embedding_dir: Default::default(),
            stacked_id_embedding_dir: Default::default(),
            input_id_images_dir: Default::default(),
            normalize_input: Default::default(),
            upscale_model_path: Default::default(),
            upscale_repeats: 1,
            weight_type: Default::default(),
            lora_model_dir: Default::default(),
            output_path: PathBuf::from("output"),
            sampling: SamplingConfig {
                steps: 20,
                cfg_scale: 7.0,
                width: 512,
                height: 512,
                seed: -1,
            },
            sampling_method: "euler_a".to_string(),
            rng_type: "cuda".to_string(),
            batch_count: 1,
            schedule_type: "discrete".to_string(),
            clip_skip: -1,
            pages: Default::default(),
            vae_tiling: Default::default(),
            vae_on_cpu: Default::default(),
            clip_on_cpu: Default::default(),
            diffusion_fa: Default::default(),
            control_net_on_cpu: Default::default(),
            canny_preprocess: Default::default(),
        }
    }
}
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub enum WeightType {
    #[default]
    None,
    F32,
    F16,
    Q4_0,
    Q4_1,
    Q5_0,
    Q5_1,
    Q8_0,
    Q2K,
    Q3K,
    Q4Ks,
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
            "--control-net",
            &self.control_net_path.to_string_lossy(),
            "--embd-dir",
            &self.embedding_dir.to_string_lossy(),
            "--stacked-id-embd-dir",
            &self.stacked_id_embedding_dir.to_string_lossy(),
            "--input-id-images-dir",
            &self.input_id_images_dir.to_string_lossy(),
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
            &self.output_path.to_string_lossy(),
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
                &self.pages.img2img.init_img_path.to_string_lossy(),
                "--strength",
                &self.pages.img2img.strength.to_string(),
            ]),
            PageType::Convert => command.args([
                "--mode",
                &PageType::Convert.to_string(),
                "--input-img",
                &self.pages.convert.input_img_path.to_string_lossy(),
            ]),
        };
        command
    }
}
