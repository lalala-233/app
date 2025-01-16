use eframe::{
    egui::{self, Context},
    App,
};
use font_kit::source::SystemSource;
use log::info;
use serde::{Deserialize, Serialize};
use std::{
    sync::{
        atomic::{AtomicBool, Ordering::Relaxed},
        Arc, Mutex,
    },
    thread::{self, sleep},
    time::Duration,
};

use crate::{
    ui::{dir_select_config, file_select_config},
    CommandBuilder, Config, ConvertPage, Img2ImgPage, PageType, Txt2ImgPage,
};

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct MyApp {
    current_page: PageType,
    config: Config,

    // 页面实例
    txt2img_page: Txt2ImgPage,
    img2img_page: Img2ImgPage,
    convert_page: ConvertPage,
    #[serde(skip)]
    is_generating: Arc<AtomicBool>,
    #[serde(skip)]
    generation_progress: f32,
    #[serde(skip)]
    last_result: Arc<Mutex<String>>,
    #[serde(skip)]
    last_error: Arc<Mutex<String>>,
}

impl MyApp {
    fn is_generating(&self) -> bool {
        self.is_generating.load(Relaxed)
    }

    fn set_fonts(ctx: &Context) {
        let system_source = SystemSource::new();
        if let Ok(family) = system_source.select_family_by_name("sans-serif") {
            if let Some(handle) = family.fonts().first().cloned() {
                if let Ok(font) = handle.load() {
                    let mut fonts = egui::FontDefinitions::default();
                    fonts.font_data.insert(
                        "system_font".to_owned(),
                        egui::FontData::from_owned(font.copy_font_data().unwrap().to_vec()).into(),
                    );
                    fonts
                        .families
                        .get_mut(&egui::FontFamily::Proportional)
                        .unwrap()
                        .insert(0, "system_font".to_owned());
                    ctx.set_fonts(fonts);
                }
            }
        }
    }
    pub fn new(cc: &eframe::CreationContext) -> Self {
        Self::set_fonts(&cc.egui_ctx);
        // 从持久化存储加载应用状态
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }
        Self::default()
    }
    fn generate_image(&mut self) {
        self.generation_progress = 0.0;
        let command_builder = CommandBuilder::new("./sd")
            .model(&self.config.diffusion_model_path)
            .vae(&self.config.vae_path)
            .seed(self.config.sampling.seed)
            .width(self.config.sampling.width)
            .height(self.config.sampling.height)
            .steps(self.config.sampling.steps)
            .cfg_scale(self.config.sampling.cfg_scale);

        let command_builder = match self.current_page {
            PageType::TextToImage => command_builder
                .mode(PageType::TextToImage)
                .prompt(&self.txt2img_page.prompt)
                .negative_prompt(&self.txt2img_page.negative_prompt)
                .output(&self.config.output_dir),
            PageType::ImageToImage => command_builder
                .mode(PageType::ImageToImage)
                .init_img(&self.img2img_page.init_img_path)
                .strength(self.img2img_page.strength)
                .output(&self.config.output_dir),
            PageType::Convert => command_builder
                .mode(PageType::Convert)
                .input_img(&self.convert_page.input_img_path)
                .output(&self.convert_page.convert_output_path),
        };
        let is_generating = Arc::clone(&self.is_generating);
        is_generating.store(true, Relaxed);
        let last_result = self.last_result.clone();
        let last_error = self.last_error.clone();
        let mut command = command_builder.build();
        info!("Args: {:?}", command.get_args());
        thread::spawn(move || {
            let output = command.output();
            is_generating.store(false, Relaxed);
            match output {
                Ok(output) => {
                    if output.status.success() {
                        let mut result = last_result.lock().unwrap();
                        *result = "图片生成成功！".to_string();
                    } else {
                        let mut error = last_error.lock().unwrap();
                        *error = String::from_utf8_lossy(&output.stderr).to_string();
                    }
                }
                Err(e) => {
                    let mut error = last_error.lock().unwrap();
                    *error = e.to_string();
                }
            };
            sleep(Duration::from_secs(5));
            last_result.lock().unwrap().clear();
            last_error.lock().unwrap().clear();
        });
    }
}

impl App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.current_page, PageType::TextToImage, "文生图");
                ui.selectable_value(&mut self.current_page, PageType::ImageToImage, "图生图");
                ui.selectable_value(&mut self.current_page, PageType::Convert, "格式转换");
            });

            ui.separator();

            ui.collapsing("通用", |ui| {
                file_select_config(
                    ui,
                    ("模型路径：", &mut self.config.diffusion_model_path),
                    ("模型文件", &["ckpt", "safetensors"]),
                );
                file_select_config(
                    ui,
                    ("VAE路径：", &mut self.config.vae_path),
                    ("VAE文件", &["pt", "ckpt"]),
                );
                dir_select_config(ui, ("输出路径：", &mut self.config.output_dir));

                ui.horizontal(|ui| {
                    ui.label("种子：");
                    ui.add(egui::DragValue::new(&mut self.config.sampling.seed));
                });
                ui.horizontal(|ui| {
                    ui.label("宽度：");
                    ui.add(egui::DragValue::new(&mut self.config.sampling.width).range(64..=2048));
                    ui.label("高度：");
                    ui.add(egui::DragValue::new(&mut self.config.sampling.height).range(64..=2048));
                });
                ui.horizontal(|ui| {
                    ui.label("步数：");
                    ui.add(egui::DragValue::new(&mut self.config.sampling.steps).range(1..=150));
                });
                ui.horizontal(|ui| {
                    ui.label("CFG Scale：");
                    ui.add(
                        egui::DragValue::new(&mut self.config.sampling.cfg_scale).range(1.0..=30.0),
                    );
                });
                ui.horizontal(|ui| {
                    let available_thread =
                        std::thread::available_parallelism().unwrap().get() as i32;
                    ui.label("线程数：");
                    ui.add(
                        egui::DragValue::new(&mut self.config.threads).range(-1..=available_thread),
                    )
                    .on_hover_text("使用的线程数（默认值：-1），<=0 时被设为 CPU 物理内核数");
                });
                ui.horizontal(|ui| {
                    ui.label("采样方法：");
                    ui.text_edit_singleline(&mut self.config.sampling_method);
                });
                ui.horizontal(|ui| {
                    ui.label("RNG 类型：");
                    ui.text_edit_singleline(&mut self.config.rng_type);
                });
                ui.horizontal(|ui| {
                    ui.label("批次数量：");
                    ui.add(egui::DragValue::new(&mut self.config.batch_count).range(1..=64));
                });
                ui.horizontal(|ui| {
                    ui.label("调度器类型：");
                    ui.text_edit_singleline(&mut self.config.schedule_type);
                });
                ui.horizontal(|ui| {
                    ui.label("CLIP skip：");
                    ui.add(egui::DragValue::new(&mut self.config.clip_skip).range(-1..=12));
                });
                ui.checkbox(&mut self.config.vae_tiling, "VAE 分块处理");
                ui.checkbox(&mut self.config.vae_on_cpu, "VAE 在 CPU");
                ui.checkbox(&mut self.config.clip_on_cpu, "CLIP 在 CPU");
                ui.checkbox(&mut self.config.diffusion_fa, "扩散模型 flash attention");
                ui.checkbox(&mut self.config.control_net_on_cpu, "ControlNet 在 CPU");
                ui.checkbox(&mut self.config.canny_preprocess, "Canny 预处理");
            });

            match self.current_page {
                PageType::TextToImage => self.txt2img_page.show(ui),
                PageType::ImageToImage => self.img2img_page.show(ui),
                PageType::Convert => self.convert_page.show(ui),
            }

            ui.separator();

            if ui.button("生成").clicked() && !self.is_generating() {
                self.generate_image();
            }

            if self.is_generating() {
                ui.label("生成中...");
                ui.spinner();
            }

            if let Ok(result) = self.last_result.try_lock() {
                if !result.is_empty() {
                    ui.label(result.as_str());
                }
            }

            if let Ok(error) = self.last_error.try_lock() {
                if !error.is_empty() {
                    ui.colored_label(egui::Color32::RED, error.as_str());
                }
            }
        });
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
}
