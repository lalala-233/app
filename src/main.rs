use app::MyApp;
fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        persist_window: true,
        ..Default::default()
    };

    eframe::run_native(
        "Stable Diffusion GUI",
        options,
        Box::new(|cc| Ok(Box::new(MyApp::new(cc)))),
    )
}
