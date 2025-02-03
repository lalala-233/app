mod control_net;
mod esrgan;
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
use crate::BigPathBuf;
use control_net::ControlNetConfig;
use eframe::egui::{Color32, Response, Ui};
use esrgan::EsrganConfig;
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
use std::{path::PathBuf, process::Command, str::FromStr};
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
    pub esrgan_config: EsrganConfig,
    pub threads: i32,
    pub model_path: BigPathBuf,
    pub _diffusion_model: BigPathBuf, // sdcpp 中支持单独指定 diffusion_model 并外接 VAE 等，不知道直接使用 model 指定是否可以外接，暂不实现
    pub clip_l_path: BigPathBuf,
    pub clip_g_path: BigPathBuf,
    pub t5xxl_path: BigPathBuf,
    pub vae_path: BigPathBuf,
    pub taesd_path: BigPathBuf,
    pub embedding_dir: BigPathBuf,
    pub lora_model_dir: BigPathBuf,
    pub batch_count: u32,
    pub output_path: BigPathBuf,
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
            esrgan_config: Default::default(),
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
            batch_count: 1,
        }
    }
}

fn file_select(ui: &mut Ui, label_name: &str, pathbuf: &mut PathBuf) -> Response {
    ui.horizontal(|ui| {
        ui.label(label_name);
        let path_str = &mut pathbuf.to_string_lossy();
        let response = ui.text_edit_singleline(path_str);
        if response.changed() {
            *pathbuf = PathBuf::from_str(path_str).unwrap_or_default()
        }
        if ui.button("选择...").clicked() {
            if let Some(path) = rfd::FileDialog::new().set_directory("./").pick_file() {
                *pathbuf = path;
            }
        }
        if !pathbuf.as_os_str().is_empty() && !pathbuf.exists() {
            ui.colored_label(Color32::RED, "文件不存在");
        }
        response
    })
    .inner
}

impl Configs {
    pub fn show(&mut self, ui: &mut Ui) {
        self.prompts.show(ui);
        self.clip_l_path.select_model(ui, "CLIP-l");
        self.clip_g_path.select_model(ui, "CLIP-g");
        self.t5xxl_path.select_model(ui, "t5xxl 模型");
        self.vae_path.select_model(ui, "VAE 模型");
        self.taesd_path.select_model(ui, "TAESD 模型");
        self.embedding_dir.select_model(ui, "embedding 模型");
        self.lora_model_dir.input_folder(ui, "LoRa 路径");
        self.control_net_config.show(ui);
        self.sampling_config.show(ui);
        self.skip_config.show(ui);
        self.photo_maker_config.show(ui);
        self.output_path.input_folder(ui, "输出路径");

        self.esrgan_config.show(ui);
        file_select(ui, "sdcpp 路径", &mut self.sdcpp_path);
        self.model_path.select_model(ui, "模型");
        self.pages_config.show(ui);
    }
    fn get_add_args(&self) -> impl Iterator<Item = &dyn AddArgs> {
        [
            self as &dyn AddArgs,
            &self.prompts,
            &self.esrgan_config,
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
