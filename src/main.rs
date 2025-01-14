use eframe::egui;
use font_kit::source::SystemSource;
use serde::{Deserialize, Serialize};
use std::sync::atomic::Ordering::Relaxed;
use std::sync::{Arc, Mutex};
use std::{process::Command, sync::atomic::AtomicBool};

mod config;
mod pages;
mod utils;

use config::Config;
use pages::{convert::ConvertPage, img2img::Img2ImgPage, txt2img::Txt2ImgPage};

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        persist_window: true,
        ..Default::default()
    };

    eframe::run_native(
        "Stable Diffusion GUI",
        options,
        Box::new(|cc| {
            let system_source = SystemSource::new();
            if let Ok(family) = system_source.select_family_by_name("sans-serif") {
                if let Some(handle) = family.fonts().first().cloned() {
                    if let Ok(font) = handle.load() {
                        let mut fonts = egui::FontDefinitions::default();
                        fonts.font_data.insert(
                            "system_font".to_owned(),
                            egui::FontData::from_owned(font.copy_font_data().unwrap().to_vec())
                                .into(),
                        );
                        fonts
                            .families
                            .get_mut(&egui::FontFamily::Proportional)
                            .unwrap()
                            .insert(0, "system_font".to_owned());
                        cc.egui_ctx.set_fonts(fonts);
                    }
                }
            }

            Ok(Box::new(MyApp::new(cc)))
        }),
    )
}

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
struct MyApp {
    #[serde(skip)]
    current_page: usize,
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

    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // 从持久化存储加载应用状态
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }
        Self::default()
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.current_page, 0, "文生图");
                ui.selectable_value(&mut self.current_page, 1, "图生图");
                ui.selectable_value(&mut self.current_page, 2, "格式转换");
            });

            ui.separator();

            ui.collapsing("通用设置", |ui| {
                ui.horizontal(|ui| {
                    ui.label("模型路径:");
                    ui.text_edit_singleline(&mut self.config.model_path);
                    if ui.button("选择...").clicked() {
                        if let Some(path) = rfd::FileDialog::new()
                            .add_filter("模型文件", &["ckpt", "safetensors"])
                            .pick_file()
                        {
                            self.config.model_path = path.to_string_lossy().to_string();
                        }
                    }
                });
                ui.horizontal(|ui| {
                    ui.label("VAE路径:");
                    let mut vae_path = self.config.vae_path.clone().unwrap_or_default();
                    ui.text_edit_singleline(&mut vae_path);
                    self.config.vae_path = Some(vae_path);
                    if ui.button("选择...").clicked() {
                        if let Some(path) = rfd::FileDialog::new()
                            .add_filter("VAE文件", &["pt", "ckpt"])
                            .pick_file()
                        {
                            self.config.vae_path = Some(path.to_string_lossy().to_string());
                        }
                    }
                });
                ui.horizontal(|ui| {
                    ui.label("输出路径:");
                    let mut path_str = self.config.output_dir.clone();
                    ui.text_edit_singleline(&mut path_str);
                    self.config.output_dir = path_str;
                });
                ui.horizontal(|ui| {
                    ui.label("种子:");
                    ui.add(egui::DragValue::new(&mut self.config.sampling.seed));
                });
                ui.horizontal(|ui| {
                    ui.label("宽度:");
                    ui.add(egui::DragValue::new(&mut self.config.sampling.width).range(64..=2048));
                    ui.label("高度:");
                    ui.add(egui::DragValue::new(&mut self.config.sampling.height).range(64..=2048));
                });
                ui.horizontal(|ui| {
                    ui.label("步数:");
                    ui.add(egui::DragValue::new(&mut self.config.sampling.steps).range(1..=150));
                });
                ui.horizontal(|ui| {
                    ui.label("CFG Scale:");
                    ui.add(
                        egui::DragValue::new(&mut self.config.sampling.cfg_scale).range(1.0..=30.0),
                    );
                });
            });

            match self.current_page {
                0 => self.txt2img_page.show(ui),
                1 => self.img2img_page.show(ui),
                2 => self.convert_page.show(ui),
                _ => unreachable!(),
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
}

impl MyApp {
    fn generate_image(&mut self) {
        self.generation_progress = 0.0;
        let lock = Arc::clone(&self.is_generating);

        let app = self.clone();
        let last_result = self.last_result.clone();
        let last_error = self.last_error.clone();

        std::thread::spawn(move || {
            let mut command = Command::new("./sd");

            command
                .arg("--model")
                .arg(&app.config.model_path)
                .arg("--vae")
                .arg(app.config.vae_path.as_ref().unwrap_or(&String::new()))
                .arg("--seed")
                .arg(app.config.sampling.seed.to_string())
                .arg("--width")
                .arg(app.config.sampling.width.to_string())
                .arg("--height")
                .arg(app.config.sampling.height.to_string())
                .arg("--steps")
                .arg(app.config.sampling.steps.to_string())
                .arg("--cfg-scale")
                .arg(app.config.sampling.cfg_scale.to_string());

            match app.current_page {
                0 => {
                    command
                        .arg("--mode")
                        .arg("txt2img")
                        .arg("--prompt")
                        .arg(&app.txt2img_page.prompt)
                        .arg("--negative-prompt")
                        .arg(&app.txt2img_page.negative_prompt);
                }
                1 => {
                    command
                        .arg("--mode")
                        .arg("img2img")
                        .arg("--init-img")
                        .arg(&app.img2img_page.init_img_path)
                        .arg("--strength")
                        .arg(app.img2img_page.strength.to_string());
                }
                2 => {
                    command
                        .arg("--mode")
                        .arg("convert")
                        .arg("--input-img")
                        .arg(&app.convert_page.input_img_path)
                        .arg("--output")
                        .arg(&app.convert_page.convert_output_path);
                }
                _ => unreachable!(),
            }
            let binding = Arc::clone(&lock);
            binding.store(true, Relaxed);
            let output = command.arg("--output").arg(&app.config.output_dir).output();
            binding.store(false, Relaxed);
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
        });
    }
}
