use crate::Configs;
use eframe::{
    egui::{Color32, ComboBox, DragValue, Response, Slider, TextEdit, Ui},
    emath,
};
use std::{ops::RangeInclusive, path::PathBuf, str::FromStr};

pub fn model_file_select(ui: &mut Ui, label: &str, file_path: &mut PathBuf) -> Response {
    file_select(
        ui,
        true,
        (label, file_path),
        (
            "模型文件",
            &["ckpt", "safetensors", "gguf", "diffusers", "pth", "sft"],
        ),
    )
}
fn folder_select(ui: &mut Ui, (label, dir_path): (&str, &mut PathBuf)) -> Response {
    file_select(ui, false, (label, dir_path), Default::default())
}
pub fn image_file_select(ui: &mut Ui, (label, dir_path): (&str, &mut PathBuf)) -> Response {
    file_select(
        ui,
        true,
        (label, dir_path),
        ("图片文件", &["png", "jpg", "jpeg", "bmp"]),
    )
}
fn select_config_combobox<T>(ui: &mut Ui, label: &str, current: &mut T) -> Response
where
    T: core::convert::AsRef<str> + Copy + PartialEq + strum::VariantArray,
{
    ui.horizontal(|ui| {
        ui.label(label);
        ComboBox::from_id_salt(label)
            .selected_text(current.as_ref())
            .show_ui(ui, |ui| {
                for &value in T::VARIANTS {
                    ui.selectable_value(current, value, value.as_ref());
                }
            })
            .response
    })
    .inner
}
pub fn slider_value<Num: emath::Numeric>(
    ui: &mut Ui,
    (label, value): (&str, &mut Num),
    range: RangeInclusive<Num>,
) -> Response {
    ui.horizontal(|ui| {
        ui.label(label);
        ui.add(Slider::new(value, range))
    })
    .inner
}
fn file_select(
    ui: &mut Ui,
    is_file: bool,
    (label_name, pathbuf): (&str, &mut PathBuf),
    (filter_name, filter): (&str, &[&str]),
) -> Response {
    ui.horizontal(|ui| {
        ui.label(label_name);
        let path_str = &mut pathbuf.to_string_lossy();
        let text_edit_builder = if is_file {
            TextEdit::singleline(path_str).hint_text(format!("后缀：{}", filter.join(", ")))
        } else {
            TextEdit::singleline(path_str)
        };
        let response = ui.add(text_edit_builder);
        let is_changed = response.changed();
        let is_clicked = ui.button("选择...").clicked();
        if is_changed {
            *pathbuf = PathBuf::from_str(path_str).unwrap_or_default()
        }
        let mut file_dialog = rfd::FileDialog::new().set_directory("./");
        if is_file {
            file_dialog = file_dialog.add_filter(filter_name, filter);
            if !pathbuf.as_os_str().is_empty() && !pathbuf.exists() {
                ui.colored_label(Color32::RED, "文件不存在");
            }
        }
        match (is_clicked, is_file) {
            (true, true) => {
                if let Some(path) = file_dialog.pick_file() {
                    *pathbuf = path;
                }
            }
            (true, false) => {
                if let Some(path) = file_dialog.pick_folder() {
                    *pathbuf = path;
                }
            }
            _ => (),
        }
        response
    })
    .inner
}

pub fn set_config(ui: &mut Ui, config: &mut Configs) {
    ui.collapsing("通用", |ui| {
        config.prompts.show(ui);
        model_file_select(ui, "CLIP-l", &mut config.clip_l_path);
        model_file_select(ui, "CLIP-g", &mut config.clip_g_path);
        model_file_select(ui, "t5xxl 模型", &mut config.t5xxl_path);
        model_file_select(ui, "VAE 模型", &mut config.vae_path);
        model_file_select(ui, "TAESD 模型", &mut config.taesd_path);
        model_file_select(ui, "embedding 模型", &mut config.embedding_dir);

        model_file_select(ui, "ESRGAN 模型", &mut config.upscale_model_path)
            .on_hover_text("仅支持 RealESRGAN_x4plus_anime_6B");
        slider_value(
            ui,
            ("超分辨率次数", &mut config.upscale_repeats),
            1..=114514,
        );
        select_config_combobox(ui, "权重类型", &mut config.weight_type)
            .on_hover_text("未指定时权重将和模型文件一致");
        folder_select(ui, ("LoRa 路径", &mut config.lora_model_dir));
        config.control_net_config.show(ui);
        config.sampling_config.show(ui);
        config.skip_config.show(ui);
        config.photo_maker_config.show(ui);
        folder_select(ui, ("输出路径", &mut config.output_path));
        ui.horizontal(|ui| {
            let available_thread = std::thread::available_parallelism().unwrap().get() as i32;
            ui.label("线程数");
            ui.add(DragValue::new(&mut config.threads).range(-1..=available_thread))
                .on_hover_text("<=0 时被设为 CPU 物理内核数");
        });
        select_config_combobox(ui, "采样方法", &mut config.sampling_method);
        select_config_combobox(ui, "RNG 类型", &mut config.rng_type);

        slider_value(ui, ("批次数量", &mut config.batch_count), 1..=64);
        select_config_combobox(ui, "调度器", &mut config.schedule_type);
        // drag_value(ui, "CLIP skip", &mut config.clip_skip, -1..=12);
        for (value, text) in config.flags.iter_mut() {
            ui.checkbox(value, text);
        }
        config.show(ui);
    });
}
