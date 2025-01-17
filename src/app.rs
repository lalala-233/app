use crate::{ui::*, Config};
use eframe::{
    egui::{self, Context, ScrollArea},
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
#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct MyApp {
    config: Config,

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
        let is_generating = Arc::clone(&self.is_generating);
        is_generating.store(true, Relaxed);
        let last_result = self.last_result.clone();
        let last_error = self.last_error.clone();
        let mut command = self.config.command();
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
            ScrollArea::vertical().show(ui, |ui| {
                select_page(ui, &mut self.config.current_page);
                ui.separator();
                set_config(ui, &mut self.config);
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
        });
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
}
