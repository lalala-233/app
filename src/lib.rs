mod app;
mod config;
mod pages;
mod ui;
pub use app::MyApp;
use config::Config;
use pages::{convert::ConvertPage, img2img::Img2ImgPage, txt2img::Txt2ImgPage, PageType};
