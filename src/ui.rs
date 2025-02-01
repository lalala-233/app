use crate::Configs;
use eframe::{
    egui::{ComboBox, DragValue, Response, Slider, Ui},
    emath,
};
use std::ops::RangeInclusive;

fn select_config_combobox<T>(ui: &mut Ui, label: &str, current: &mut T) -> Response
where
    T: AsRef<str> + Copy + PartialEq + strum::VariantArray,
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

pub fn set_config(ui: &mut Ui, config: &mut Configs) {
    ui.collapsing("通用", |ui| {
        config.prompts.show(ui);
        config.clip_l_path.select_model(ui, "CLIP-l");
        config.clip_g_path.select_model(ui, "CLIP-g");
        config.t5xxl_path.select_model(ui, "t5xxl 模型");
        config.vae_path.select_model(ui, "VAE 模型");
        config.taesd_path.select_model(ui, "TAESD 模型");
        config.embedding_dir.select_model(ui, "embedding 模型");

        select_config_combobox(ui, "权重类型", &mut config.weight_type)
            .on_hover_text("未指定时权重将和模型文件一致");
        config.lora_model_dir.select_fold(ui, "LoRa 路径");
        config.control_net_config.show(ui);
        config.sampling_config.show(ui);
        config.skip_config.show(ui);
        config.photo_maker_config.show(ui);
        config.output_path.select_fold(ui, "输出路径");
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
        for (value, text) in config.flags.iter_mut() {
            ui.checkbox(value, text);
        }
        config.show(ui);
    });
}
