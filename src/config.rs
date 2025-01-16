use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Config {
    /// 模型文件路径
    pub diffusion_model_path: String,
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
    pub txt2img: Txt2ImgConfig,
    pub img2img: Img2ImgConfig,
    pub convert: ConvertConfig,
}

/// 文生图页面配置
#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct Txt2ImgConfig {
    pub default_prompt: String,
    pub default_negative_prompt: String,
}

/// 图生图页面配置
#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct Img2ImgConfig {
    pub default_denoising_strength: f32,
}
/// 格式转换页面配置
#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct ConvertConfig {
    pub default_output_format: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            diffusion_model_path: "model.safetensors".to_string(),
            vae_path: String::new(),
            output_dir: "output".to_string(),
            sampling: SamplingConfig {
                steps: 20,
                cfg_scale: 7.0,
                width: 512,
                height: 512,
                seed: -1,
            },
            pages: PagesConfig {
                txt2img: Txt2ImgConfig {
                    default_prompt: String::new(),
                    default_negative_prompt: String::new(),
                },
                img2img: Img2ImgConfig {
                    default_denoising_strength: 0.75,
                },
                convert: ConvertConfig {
                    default_output_format: "png".to_string(),
                },
            },
            // 新增字段默认值
            threads: -1,
            clip_l_path: None,
            clip_g_path: None,
            t5xxl_path: None,
            taesd_path: None,
            control_net_path: None,
            embedding_dir: None,
            upscale_model_path: None,
            lora_model_dir: None,
            sampling_method: "euler_a".to_string(),
            rng_type: "cuda".to_string(),
            batch_count: 1,
            schedule_type: "discrete".to_string(),
            clip_skip: -1,
            vae_tiling: false,
            vae_on_cpu: false,
            clip_on_cpu: false,
            diffusion_fa: false,
            control_net_on_cpu: false,
            canny_preprocess: false,
        }
    }
}
