use crate::{Config, PageType};
use eframe::{
    egui::{ComboBox, DragValue, Response, TextEdit, Ui},
    emath,
};
use std::{ops::RangeInclusive, path::PathBuf, str::FromStr};
pub fn select_page(ui: &mut Ui, current_page: &mut PageType) {
    ui.horizontal(|ui| {
        ui.selectable_value(current_page, PageType::Txt2Img, "文生图");
        ui.selectable_value(current_page, PageType::Img2Img, "图生图");
        ui.selectable_value(current_page, PageType::Convert, "格式转换");
    });
}
fn model_file_select(ui: &mut Ui, label: &str, file_path: &mut PathBuf) -> Response {
    let (filter_name, filter) = (
        "模型文件",
        &["ckpt", "safetensors", "gguf", "diffusers", "pth", "sft"],
    );
    file_select(ui, true, (label, file_path), (filter_name, filter))
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
pub fn drag_value<Num: emath::Numeric>(
    ui: &mut Ui,
    label: &str,
    value: &mut Num,
    range: RangeInclusive<Num>,
) -> Response {
    ui.horizontal(|ui| {
        ui.label(label);
        ui.add(DragValue::new(value).range(range))
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
        if let Some(ext) = pathbuf.extension() {
            if filter.contains(&ext.to_string_lossy().as_ref()) && pathbuf.is_file() {
                ui.label("文件存在");
            }
        }
        response
    })
    .inner
}

pub fn set_config(ui: &mut Ui, config: &mut Config) {
    ui.collapsing("通用", |ui| {
        model_file_select(ui, "CLIP-l", &mut config.clip_l_path);
        model_file_select(ui, "CLIP-g", &mut config.clip_g_path);
        model_file_select(ui, "t5xxl 模型", &mut config.t5xxl_path);
        model_file_select(ui, "VAE 模型", &mut config.vae_path);
        model_file_select(ui, "TAESD 模型", &mut config.taesd_path);
        model_file_select(ui, "Control Net 模型", &mut config.control_net_path);
        model_file_select(ui, "embedding 模型", &mut config.embedding_dir);
        model_file_select(ui, "PhotoMaker 模型", &mut config.stacked_id_embedding_dir);
        model_file_select(ui, "PhotoMaker 输入图片", &mut config.input_id_images_dir);
        model_file_select(ui, "ESRGAN 模型", &mut config.upscale_model_path)
            .on_hover_text("仅支持 RealESRGAN_x4plus_anime_6B");
        drag_value(ui, "超分辨率次数", &mut config.upscale_repeats, 1..=114514);
        select_config_combobox(ui, "权重类型", &mut config.weight_type)
            .on_hover_text("未指定时权重将和模型文件一致");
        folder_select(ui, ("LoRa 路径", &mut config.lora_model_dir));
        image_file_select(ui, ("Control Net 图像", &mut config.control_net_image));
        folder_select(ui, ("输出路径", &mut config.output_path));
        drag_value(ui, "种子", &mut config.sampling.seed, -1..=1145141919810);
        ui.horizontal(|ui| {
            ui.label("宽度：");
            ui.add(
                DragValue::new(&mut config.sampling.width)
                    .range(64..=2048)
                    .speed(64),
            );
            ui.label("高度");
            ui.add(
                DragValue::new(&mut config.sampling.height)
                    .range(64..=2048)
                    .speed(64),
            );
        });
        drag_value(ui, "CFG Scale", &mut config.sampling.cfg_scale, 0.1..=30.0);
        drag_value(ui, "SLG Scale", &mut config.sampling.slg_scale, 0.1..=30.0)
            .on_hover_text("仅适用于 DiT 模型（默认值：0） ");
        drag_value(ui, "步数", &mut config.sampling.steps, 1..=150);
        ui.horizontal(|ui| {
            let available_thread = std::thread::available_parallelism().unwrap().get() as i32;
            ui.label("线程数");
            ui.add(DragValue::new(&mut config.threads).range(-1..=available_thread))
                .on_hover_text("<=0 时被设为 CPU 物理内核数");
        });
        select_config_combobox(ui, "采样方法", &mut config.sampling_method);
        select_config_combobox(ui, "RNG 类型", &mut config.rng_type);

        drag_value(ui, "批次数量", &mut config.batch_count, 1..=64);
        select_config_combobox(ui, "调度器", &mut config.schedule_type);
        drag_value(ui, "CLIP skip", &mut config.clip_skip, -1..=12);
        for (value, text) in config.flags.iter_mut() {
            ui.checkbox(value, text);
        }
    });

    match config.current_page {
        PageType::Txt2Img => {
            model_file_select(ui, "模型", &mut config.model_path);
            config.pages.txt2img.show(ui)
        }
        PageType::Img2Img => {
            model_file_select(ui, "模型", &mut config.model_path);
            config.pages.img2img.show(ui)
        }
        PageType::Convert => {
            model_file_select(ui, "待转换模型", &mut config.model_path);
            config.pages.convert.show(ui)
        }
    }
}
