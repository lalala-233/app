pub mod convert;
pub mod img2img;
pub mod txt2img;
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
