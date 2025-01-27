pub mod convert;
pub mod img2img;
pub mod txt2img;
use super::AddArgs;
use convert::ConvertPage;
use eframe::egui::Ui;
use img2img::Img2ImgPage;
use serde::{Deserialize, Serialize};
use strum::{AsRefStr, VariantArray};
use txt2img::Txt2ImgPage;

/// 页面配置
#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct PagesConfig {
    current_page: PageType,
    txt2img: Txt2ImgPage,
    img2img: Img2ImgPage,
    convert: ConvertPage,
}
impl PagesConfig {
    pub fn select_page(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.selectable_value(&mut self.current_page, PageType::Txt2Img, "文生图");
            ui.selectable_value(&mut self.current_page, PageType::Img2Img, "图生图");
            ui.selectable_value(&mut self.current_page, PageType::Convert, "格式转换");
        });
    }
    pub fn show(&mut self, ui: &mut Ui) {
        match self.current_page {
            PageType::Txt2Img => self.txt2img.show(ui),
            PageType::Img2Img => self.img2img.show(ui),
            PageType::Convert => self.convert.show(ui),
        }
    }
}
impl AddArgs for PagesConfig {
    fn add_args(&self, command: &mut std::process::Command) {
        command.args(["--mode", self.current_page.as_ref()]);
        match self.current_page {
            PageType::Txt2Img => (),
            PageType::Convert => (),
            PageType::Img2Img => self.img2img.add_args(command),
        }
    }
}
#[derive(
    Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Default, AsRefStr, VariantArray,
)]
#[strum(serialize_all = "lowercase")]
enum PageType {
    #[default]
    Txt2Img,
    Img2Img,
    Convert,
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
