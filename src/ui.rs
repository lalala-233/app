use crate::Configs;
use eframe::{
    egui::{ComboBox, Response, Slider, Ui},
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
    label: &str,
    value: &mut Num,
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
        let available_thread = std::thread::available_parallelism().unwrap().get() as i32;
        slider_value(ui, "线程数", &mut config.threads, -1..=available_thread)
            .on_hover_text("<=0 时被设为 CPU 物理内核数");
        slider_value(ui, "批次数量", &mut config.batch_count, 1..=64);
        select_config_combobox(ui, "权重类型", &mut config.weight_type)
            .on_hover_text("未指定时权重将和模型文件一致");
        select_config_combobox(ui, "采样方法", &mut config.sampling_method);
        select_config_combobox(ui, "RNG 类型", &mut config.rng_type);
        select_config_combobox(ui, "调度器", &mut config.schedule_type);
        for (value, text) in config.flags.iter_mut() {
            ui.checkbox(value, text);
        }
        config.show(ui);
    });
}
