use crate::{Config, PageType};
use eframe::egui::{DragValue, TextEdit, Ui};
use std::{path::PathBuf, str::FromStr};
pub fn select_page(ui: &mut Ui, current_page: &mut PageType) {
    ui.horizontal(|ui| {
        ui.selectable_value(current_page, PageType::TextToImage, "文生图");
        ui.selectable_value(current_page, PageType::ImageToImage, "图生图");
        ui.selectable_value(current_page, PageType::Convert, "格式转换");
    });
}
fn model_select_config(ui: &mut Ui, label_name: &str, file_path: &mut PathBuf) {
    let (filter_name, filter) = (
        "模型文件",
        &["ckpt", "safetensors", "gguf", "diffusers", "pth"],
    );
    select_config(ui, true, (label_name, file_path), (filter_name, filter));
}
fn dir_select_config(ui: &mut Ui, (label_name, dir_path): (&str, &mut PathBuf)) {
    select_config(ui, false, (label_name, dir_path), ("", &["png"]));
}
pub fn select_config(
    ui: &mut Ui,
    is_file: bool,
    (label_name, pathbuf): (&str, &mut PathBuf),
    (filter_name, filter): (&str, &[&str]),
) {
    ui.horizontal(|ui| {
        ui.label(label_name);
        let path_str = &mut pathbuf.to_string_lossy();
        let text_edit_builder = if is_file {
            TextEdit::singleline(path_str).hint_text(format!("后缀：{}", filter.join(", ")))
        } else {
            TextEdit::singleline(path_str)
        };
        let is_changed = ui.add(text_edit_builder).changed();
        let is_clicked = ui.button("选择...").clicked();
        if is_changed {
            *pathbuf = PathBuf::from_str(path_str).unwrap_or_default()
        }
        let file_dialog = if is_file {
            rfd::FileDialog::new()
                .set_directory("./")
                .add_filter(filter_name, filter)
        } else {
            rfd::FileDialog::new().set_directory("./")
        };
        if is_clicked {
            if let Some(path) = file_dialog.pick_file() {
                *pathbuf = path;
            }
        }
        if let Some(ext) = pathbuf.extension() {
            if filter.contains(&ext.to_string_lossy().as_ref()) && pathbuf.is_file() {
                ui.label("文件存在");
            }
        }
    });
}

pub fn set_config(ui: &mut Ui, config: &mut Config) {
    ui.collapsing("通用", |ui| {
        model_select_config(ui, "clip-l 路径：", &mut config.clip_l_path);
        model_select_config(ui, "clip-g 路径：", &mut config.clip_g_path);
        model_select_config(ui, "t5xxl 路径：", &mut config.t5xxl_path);
        model_select_config(ui, "VAE 路径：", &mut config.vae_path);
        model_select_config(ui, "TAESD 路径：", &mut config.taesd_path);
        model_select_config(ui, "embedding 路径：", &mut config.embedding_dir);
        model_select_config(
            ui,
            "PhotoMaker 路径：",
            &mut config.stacked_id_embedding_dir,
        );
        model_select_config(ui, "PhotoMaker 输入图片：", &mut config.input_id_images_dir);
        dir_select_config(ui, ("输出路径：", &mut config.output_path));

        ui.horizontal(|ui| {
            ui.label("种子：");
            ui.add(DragValue::new(&mut config.sampling.seed));
        });
        ui.horizontal(|ui| {
            ui.label("宽度：");
            ui.add(
                DragValue::new(&mut config.sampling.width)
                    .range(64..=2048)
                    .speed(64),
            );
            ui.label("高度：");
            ui.add(
                DragValue::new(&mut config.sampling.height)
                    .range(64..=2048)
                    .speed(64),
            );
        });
        ui.horizontal(|ui| {
            ui.label("步数：");
            ui.add(DragValue::new(&mut config.sampling.steps).range(1..=150));
        });
        ui.horizontal(|ui| {
            ui.label("CFG Scale：");
            ui.add(DragValue::new(&mut config.sampling.cfg_scale).range(0.0..=30.0));
        });
        ui.horizontal(|ui| {
            let available_thread = std::thread::available_parallelism().unwrap().get() as i32;
            ui.label("线程数：");
            ui.add(DragValue::new(&mut config.threads).range(-1..=available_thread))
                .on_hover_text("使用的线程数（默认值：-1），<=0 时被设为 CPU 物理内核数");
        });
        ui.horizontal(|ui| {
            ui.label("采样方法：");
            ui.text_edit_singleline(&mut config.sampling_method);
        });
        ui.horizontal(|ui| {
            ui.label("RNG 类型：");
            ui.text_edit_singleline(&mut config.rng_type);
        });
        ui.horizontal(|ui| {
            ui.label("批次数量：");
            ui.add(DragValue::new(&mut config.batch_count).range(1..=64));
        });
        ui.horizontal(|ui| {
            ui.label("调度器类型：");
            ui.text_edit_singleline(&mut config.schedule_type);
        });
        ui.horizontal(|ui| {
            ui.label("CLIP skip：");
            ui.add(DragValue::new(&mut config.clip_skip).range(-1..=12));
        });
        ui.checkbox(&mut config.vae_tiling, "VAE 分块处理");
        ui.checkbox(&mut config.vae_on_cpu, "VAE 在 CPU");
        ui.checkbox(&mut config.clip_on_cpu, "CLIP 在 CPU");
        ui.checkbox(&mut config.diffusion_fa, "扩散模型 flash attention");
        ui.checkbox(&mut config.control_net_on_cpu, "ControlNet 在 CPU");
        ui.checkbox(&mut config.canny_preprocess, "Canny 预处理");
    });

    match config.current_page {
        PageType::TextToImage => {
            model_select_config(ui, "模型路径：", &mut config.model_path);
            config.pages.txt2img.show(ui)
        }
        PageType::ImageToImage => {
            model_select_config(ui, "模型路径：", &mut config.model_path);
            config.pages.img2img.show(ui)
        }
        PageType::Convert => {
            model_select_config(ui, "待转换模型：", &mut config.model_path);
            config.pages.convert.show(ui)
        }
    }
}
