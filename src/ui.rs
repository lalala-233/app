use crate::PageType;
use eframe::egui::{TextEdit, Ui};
use std::{path::PathBuf, str::FromStr};

pub fn file_select_config(
    ui: &mut Ui,
    (label_name, file_path): (&str, &mut PathBuf),
    (filter_name, filter): (&str, &[&str]),
) {
    ui.horizontal(|ui| {
        ui.label(label_name);
        let file_path_str = &mut file_path.to_string_lossy();
        ui.add(
            TextEdit::singleline(file_path_str).hint_text(format!("后缀：{}", filter.join(", "))),
        );
        *file_path = PathBuf::from_str(file_path_str).unwrap_or_default();
        if ui.button("选择...").clicked() {
            if let Some(path) = rfd::FileDialog::new()
                .set_directory("./")
                .add_filter(filter_name, filter)
                .pick_file()
            {
                *file_path = path;
            }
        }
        if let Some(ext) = file_path.extension() {
            if file_path.is_file() && filter.contains(&ext.to_string_lossy().as_ref()) {
                ui.label("文件存在");
            }
        }
    });
}

pub fn dir_select_config(ui: &mut Ui, (label_name, dir_path): (&str, &mut PathBuf)) {
    ui.horizontal(|ui| {
        ui.label(label_name);
        ui.text_edit_singleline(&mut dir_path.to_string_lossy());
        if ui.button("选择...").clicked() {
            if let Some(path) = rfd::FileDialog::new().set_directory("./").pick_folder() {
                *dir_path = path;
            }
        }
    });
}

pub fn select_page(ui: &mut Ui, current_page: &mut PageType) {
    ui.horizontal(|ui| {
        ui.selectable_value(current_page, PageType::TextToImage, "文生图");
        ui.selectable_value(current_page, PageType::ImageToImage, "图生图");
        ui.selectable_value(current_page, PageType::Convert, "格式转换");
    });
}
