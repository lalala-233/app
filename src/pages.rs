pub mod convert;
pub mod img2img;
pub mod txt2img;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Default)]
pub enum PageType {
    #[default]
    TextToImage,
    ImageToImage,
    Convert,
}

impl Display for PageType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let convert = match self {
            PageType::TextToImage => "txt2img",
            PageType::ImageToImage => "img2img",
            PageType::Convert => "convert",
        };
        f.write_str(convert)
    }
}
