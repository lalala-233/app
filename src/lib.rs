mod app;
mod command_builder;
mod config;
mod pages;
pub use app::MyApp;
use command_builder::CommandBuilder;
use config::Config;
use pages::{convert::ConvertPage, img2img::Img2ImgPage, txt2img::Txt2ImgPage, PageType};
