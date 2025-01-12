use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Config {
    /// 模型文件路径
    pub model_path: String,
    /// VAE文件路径
    pub vae_path: Option<String>,
    /// 输出目录
    pub output_dir: String,
    /// 默认采样参数
    pub sampling: SamplingConfig,
    /// 页面配置
    pub pages: PagesConfig,
}

/// 采样参数配置
#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct SamplingConfig {
    pub steps: u32,
    pub cfg_scale: f32,
    pub width: u32,
    pub height: u32,
    pub seed: u64,
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
            model_path: "model.safetensors".to_string(),
            vae_path: None,
            output_dir: "output".to_string(),
            sampling: SamplingConfig {
                steps: 20,
                cfg_scale: 7.0,
                width: 512,
                height: 512,
                seed: 42,
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
        }
    }
}
