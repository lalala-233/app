use eframe::egui::Ui;

pub fn file_select_config(
    ui: &mut Ui,
    (lable_name, file_path): (&str, &mut String),
    (filter_name, filter): (&str, &[&str]),
) {
    ui.horizontal(|ui| {
        ui.label(lable_name);
        ui.text_edit_singleline(file_path);
        if ui.button("选择...").clicked() {
            if let Some(path) = rfd::FileDialog::new()
                .set_directory("./")
                .add_filter(filter_name, filter)
                .pick_file()
            {
                *file_path = path.to_string_lossy().to_string();
            }
        }
    });
}
pub fn dir_select_config(ui: &mut Ui, (lable_name, dir_path): (&str, &mut String)) {
    ui.horizontal(|ui| {
        ui.label(lable_name);
        ui.text_edit_singleline(dir_path);
        if ui.button("选择...").clicked() {
            if let Some(path) = rfd::FileDialog::new().set_directory("./").pick_folder() {
                *dir_path = path.to_string_lossy().to_string();
            }
        }
    });
}
