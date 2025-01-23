pub mod convert;
pub mod img2img;
pub mod txt2img;
use serde::{Deserialize, Serialize};
use strum::AsRefStr;
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Default, AsRefStr)]
#[strum(serialize_all = "lowercase")]
pub enum PageType {
    #[default]
    Txt2Img,
    Img2Img,
    Convert,
}
