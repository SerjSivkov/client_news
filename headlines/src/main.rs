mod headlines;
use eframe::{NativeOptions, egui::{CentralPanel, ScrollArea, Vec2, Ui, Separator, TopBottomPanel, CtxRef, Label, Hyperlink}, epi::App, run_native};
use headlines::{Headlines, PADDING};

impl App for Headlines {
    fn setup(&mut self, ctx: &eframe::egui::CtxRef, _frame: &mut eframe::epi::Frame<'_>, _storage: Option<&dyn eframe::epi::Storage>) {
        self.configure_fonts(ctx);
    }

    fn update(&mut self, ctx: &eframe::egui::CtxRef, frame: &mut eframe::epi::Frame<'_>) {
        self.render_top_panel(ctx);
        CentralPanel::default().show(ctx, |ui| {
            render_header(ui);
            ScrollArea::auto_sized().show(ui, |ui| { // Добавление скрола
                self.render_news_cards(ui);
            });
            render_footer(ctx)
        });
    }

    fn name(&self) -> &str {
        "Headlines"
    }
}

fn render_footer(ctx: &CtxRef) {
    TopBottomPanel::bottom("footer").show(ctx, |ui| {
        ui.vertical_centered(|ui| {
            ui.add_space(10.);
            ui.add(Label::new("API source: newsapi.org").monospace());
            ui.add(Hyperlink::new("https://github.com/emilk/egui").text("Made with egui").text_style(eframe::egui::TextStyle::Monospace));
            ui.add(Hyperlink::new("https://github.com/SerjSivkov/headlines").text("serjsivkov/headlines").text_style(eframe::egui::TextStyle::Monospace));
            ui.add_space(10.)
        })
    });
}
fn render_header(ui: &mut Ui) {
    ui.vertical_centered(|ui| {
        ui.heading("headlines")
    });
    ui.add_space(PADDING);
    let sep = Separator::default().spacing(20.);
    ui.add(sep);
}

fn main() {
    let app = Headlines::new();
    let mut window_option = NativeOptions::default();
    window_option.initial_window_size = Some(Vec2::new(540., 920.));
    run_native(Box::new(app), window_option)
}