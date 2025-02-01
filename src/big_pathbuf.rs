use eframe::egui::{ComboBox, Response, Ui};
use serde::{Deserialize, Serialize};
use std::{borrow::Cow, path::PathBuf};
#[derive(Serialize, Deserialize, Clone, Debug, Default)]

pub struct BigPathBuf {
    current: PathBuf,
    cached_options: Vec<(PathBuf, String)>,
    needs_refresh: bool,
}
impl BigPathBuf {
    const MODEL_EXTENSION: &[&str] = &["ckpt", "safetensors", "gguf", "diffusers", "pth", "sft"];
    const IMAGE_EXTENSION: &[&str] = &["png", "jpg", "jpeg", "bmp"];
    fn refresh_options(&mut self, filters: &[&str]) {
        if !self.needs_refresh || filters.is_empty() {
            return;
        }
        let mut path = self.current.as_path();
        if path.is_file() {
            if let Some(parent) = path.parent() {
                path = parent
            } else {
                return;
            };
        }
        self.cached_options = match path.read_dir() {
            Ok(entries) => entries
                .flatten()
                .filter(|entry| {
                    entry
                        .file_type()
                        .ok()
                        .map(|file_type| file_type.is_file())
                        .unwrap_or(false) // 过滤非文件
                })
                .filter_map(|entry| {
                    let path = entry.path();
                    let file_name = path.file_name()?.to_string_lossy().into_owned();
                    let extension = path.extension()?.to_string_lossy();
                    if !filters.contains(&extension.as_ref()) {
                        return None;
                    }

                    Some((path, file_name))
                })
                .collect(),
            Err(_) => Vec::new(),
        };

        self.needs_refresh = false;
    }
    pub fn to_string_lossy(&self) -> Cow<'_, str> {
        self.current.to_string_lossy()
    }
    pub fn select_fold(&mut self, ui: &mut Ui, label: &str) -> Response {
        self.show(ui, label, Default::default())
    }
    pub fn select_model(&mut self, ui: &mut Ui, label: &str) -> Response {
        self.show(ui, label, Self::MODEL_EXTENSION)
    }
    pub fn select_image(&mut self, ui: &mut Ui, label: &str) -> Response {
        self.show(ui, label, Self::IMAGE_EXTENSION)
    }
    fn set_path(&mut self, new_path: PathBuf) {
        if self.current != new_path {
            self.needs_refresh = true;
            self.current = new_path;
        }
    }

    fn show(&mut self, ui: &mut Ui, label: &str, filters: &[&str]) -> Response {
        ui.horizontal(|ui| {
            ui.label(label);
            let path_label = self
                .current
                .file_name()
                .map(|s| s.to_string_lossy())
                .unwrap_or_default();

            let combo = ComboBox::from_id_salt(label)
                .selected_text(path_label)
                .show_ui(ui, |ui| {
                    self.refresh_options(filters);
                    for (path, name) in &self.cached_options {
                        ui.selectable_value(&mut self.current, path.clone(), name);
                    }
                });

            if ui.button("选择...").clicked() {
                if let Some(new_path) = rfd::FileDialog::new().pick_folder() {
                    self.set_path(new_path);
                }
            }

            combo.response
        })
        .inner
    }
}
