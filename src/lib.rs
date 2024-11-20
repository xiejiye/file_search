pub mod gui;
pub mod search;
pub mod os;

pub fn run() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "文件快搜",
        options,
        Box::new(|cc| {
            gui::configure_fonts(&cc.egui_ctx); // 配置字体
            gui::configure_theme(&cc.egui_ctx);
            Box::new(gui::FileSearchApp::default())
        }),
    )
}