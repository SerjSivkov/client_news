use eframe::{NativeOptions, egui::Vec2, run_native};
use headlines::Headlines;

fn main() {
    tracing_subscriber::fmt::init();
    // tracing::error!("this is a log");
    let app = Headlines::new();
    let mut window_option = NativeOptions::default();
    window_option.initial_window_size = Some(Vec2::new(540., 920.));
    run_native(Box::new(app), window_option)
}