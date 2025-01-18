pub mod convert;
pub mod img2img;
pub mod txt2img;
use serde::{Deserialize, Serialize};
use strum::AsRefStr;
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Default, AsRefStr)]
pub enum PageType {
    #[default]
    TextToImage,
    ImageToImage,
    Convert,
}
