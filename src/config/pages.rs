pub mod convert;
pub mod img2img;
pub mod txt2img;
use eframe::egui::Ui;
use serde::{Deserialize, Serialize};
use strum::{AsRefStr, VariantArray};
#[derive(
    Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Default, AsRefStr, VariantArray,
)]
#[strum(serialize_all = "lowercase")]
pub enum PageType {
    #[default]
    Txt2Img,
    Img2Img,
    Convert,
}
impl PageType {
    pub fn select_page(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.selectable_value(self, PageType::Txt2Img, "文生图");
            ui.selectable_value(self, PageType::Img2Img, "图生图");
            ui.selectable_value(self, PageType::Convert, "格式转换");
        });
    }
}
#[cfg(test)]
mod test {
    use super::PageType;
    use strum::VariantArray;
    #[test]
    fn as_str() {
        let expect = ["txt2img", "img2img", "convert"].into_iter();
        let actual = PageType::VARIANTS.iter().map(|v| v.as_ref());
        for (expect, actual) in expect.zip(actual) {
            assert_eq!(expect, actual)
        }
    }
}
