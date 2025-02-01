use crate::{ui::*, Configs};
use eframe::{
    egui::{self, Context, ScrollArea},
    App,
};
use font_kit::source::SystemSource;
use log::info;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::{
    io::Read,
    process::Stdio,
    sync::{
        atomic::{AtomicBool, AtomicU32, Ordering::Relaxed},
        Arc, Mutex,
    },
    thread::{self, sleep},
    time::Duration,
};
#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct MyApp {
    config: Configs,
    #[serde(skip)]
    is_generating: Arc<AtomicBool>,
    #[serde(skip)]
    progress: Arc<(AtomicU32, AtomicU32)>,
    #[serde(skip)]
    last_result: Arc<Mutex<String>>,
    #[serde(skip)]
    last_error: Arc<Mutex<String>>,
}

impl MyApp {
    fn is_generating(&self) -> bool {
        self.is_generating.load(Relaxed)
    }
    fn get_progress(&self) -> (u32, u32) {
        (self.progress.0.load(Relaxed), self.progress.1.load(Relaxed))
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
        let mut style = egui::Style::default();
        style.interaction.tooltip_delay = 0.1;
        cc.egui_ctx.set_style(style);
        Self::set_fonts(&cc.egui_ctx);
        // 从持久化存储加载应用状态
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }
        Self::default()
    }
    fn generate_image(&mut self) {
        let is_generating = self.is_generating.clone();
        let last_result = self.last_result.clone();
        let last_error = self.last_error.clone();
        let progress = self.progress.clone();

        is_generating.store(true, Relaxed);
        let mut command = self.config.command();
        info!("Args: {:?}", command.get_args());

        thread::spawn(move || {
            let mut child = match command
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
            {
                Ok(c) => c,
                Err(e) => {
                    is_generating.store(false, Relaxed);
                    *last_error.lock().unwrap() = e.to_string();
                    return;
                }
            };
            let stdout = &mut child.stdout.take().unwrap();
            let stderr = &mut child.stderr.take().unwrap();

            let re = Regex::new(r"(\d+)/(\d+)").unwrap();
            loop {
                if let Some(s) = detect_progress(stdout) {
                    if let Some(caps) = re.captures(&s) {
                        progress.0.store(caps[1].parse::<u32>().unwrap(), Relaxed);
                        progress.1.store(caps[2].parse::<u32>().unwrap(), Relaxed);
                    }
                }
                match child.try_wait() {
                    Ok(Some(status)) => {
                        if status.success() {
                            let mut result = last_result.lock().unwrap();
                            *result = "图片生成成功！".to_string();
                        } else {
                            let mut error = last_error.lock().unwrap();
                            let _ = stderr.read_to_string(&mut error);
                        }
                        break;
                    }
                    Ok(None) if !is_generating.load(Relaxed) => {
                        // 取消生成
                        let _ = child.kill().map_err(|e| {
                            let mut error = last_error.lock().unwrap();
                            *error = e.to_string();
                        });
                        break;
                    }
                    Ok(None) => {
                        // 添加短暂休眠减少 CPU 占用
                        sleep(Duration::from_millis(100));
                    }
                    Err(e) => {
                        let mut error = last_error.lock().unwrap();
                        *error = e.to_string();
                        break;
                    }
                }
            }

            sleep(Duration::from_secs(10));
            last_result.lock().unwrap().clear();
            last_error.lock().unwrap().clear();
            progress.0.store(0, Relaxed);
            progress.1.store(0, Relaxed);
        });
    }
}

impl App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ScrollArea::vertical().show(ui, |ui| {
                self.config.pages_config.select_page(ui);
                ui.separator();
                set_config(ui, &mut self.config);
                ui.separator();
                if self.is_generating() {
                    if ui.button("取消").clicked() {
                        self.is_generating.store(false, Relaxed);
                    };
                    ui.label("生成中...");
                    let (step, steps) = self.get_progress();
                    if steps != 0 {
                        ui.label(format!("{}/{}", step, steps));
                    }
                } else if ui.button("生成").clicked() {
                    self.generate_image();
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
                egui::widgets::global_theme_preference_switch(ui)
            });
        });
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
}

fn detect_progress<R: Read>(reader: &mut R) -> Option<String> {
    let mut buf = [0u8; 1024];
    loop {
        let bytes_read = reader.read(&mut buf).unwrap_or_default();
        if bytes_read == 0 {
            return None;
        }

        let buf = &buf[..bytes_read];
        if !buf.contains(&b'\n') {
            return Some(String::from_utf8(buf.to_vec()).unwrap_or_default());
        }
    }
}
