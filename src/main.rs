fn main() -> eframe::Result<()> {
    env_logger::init();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("⏱ Time Warp Rusted")
            .with_inner_size([1400.0, 900.0])
            .with_min_inner_size([800.0, 600.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Time Warp Rusted",
        options,
        Box::new(|cc| Ok(Box::new(tw_ui::TimeWarpApp::new(cc)))),
    )
}
