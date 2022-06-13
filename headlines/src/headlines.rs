use std::borrow::Cow;
use eframe::egui::{CtxRef, FontDefinitions, FontFamily, Color32, Label, Layout, Hyperlink, Separator, TopBottomPanel, self, Button};

pub const PADDING: f32 = 5.0;
const WHITE: Color32 = Color32::from_rgb(255, 255, 255);
const BLACK: Color32 = Color32::from_rgb(0, 0, 0);
const CYAN: Color32 = Color32::from_rgb(0, 255, 255);
const RED: Color32 = Color32::from_rgb(255, 0, 0);


pub struct HeadlinesConfig {
    pub dark_mode: bool,
}

impl HeadlinesConfig {
    pub fn new() -> Self {
        Self {
            dark_mode: true
        }
    }
}

pub struct Headlines {
    articles: Vec<NewsCardData>,
    pub config: HeadlinesConfig
}

struct NewsCardData {
    title: String,
    desc: String,
    url: String
}

impl Headlines {
    pub fn new() -> Headlines {
        let iter = (0..20).map(|article| NewsCardData {
            title: format!("title{}", article),
            desc: format!("desc{}", article),
            url: format!("http://example.com/{}", article)
        });
        Headlines {
            articles: Vec::from_iter(iter),
            config: HeadlinesConfig::new()
        }
    }

    pub fn configure_fonts(&self, ctx: &CtxRef) {
        let mut font_def = FontDefinitions::default();
        font_def.font_data.insert("MesloLGS".to_string(), Cow::Borrowed(include_bytes!("../../MesloLGS_NF_Regular.ttf")));
        font_def.family_and_size.insert(eframe::egui::TextStyle::Heading, (FontFamily::Proportional, 35.));
        font_def.family_and_size.insert(eframe::egui::TextStyle::Body, (FontFamily::Proportional, 20.));
        font_def.fonts_for_family.get_mut(&FontFamily::Proportional).unwrap().insert(0, "MesloLGS".to_string());
        ctx.set_fonts(font_def);
    }

    pub fn render_news_cards(&self, ui: &mut eframe::egui::Ui) {
        for a in &self.articles {
            ui.add_space(PADDING);
            // render title
            let title = format!("▶ {}", a.title);
            if self.config.dark_mode {
                ui.colored_label(WHITE, title);
            } else {
                ui.colored_label(BLACK, title);
            }
            // render desc
            ui.add_space(PADDING);
            let desc = Label::new(&a.desc).text_style(eframe::egui::TextStyle::Button);
            ui.add(desc);

            // render hyperlinks
            if self.config.dark_mode {
                ui.style_mut().visuals.hyperlink_color = CYAN;
            } else {
                ui.style_mut().visuals.hyperlink_color = RED;
            }
            ui.add_space(PADDING);
            ui.with_layout(Layout::right_to_left(), |ui| {
                ui.add(Hyperlink::new(&a.url).text("read more ⤴"));
            });
            ui.add_space(PADDING);
            ui.add(Separator::default());
        }
    }

    pub(crate) fn render_top_panel(&mut self, ctx: &CtxRef, frame: &mut eframe::epi::Frame<'_>) {
        TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.add_space(10.);
            egui::menu::bar(ui, |ui| {
                // logo
                ui.with_layout(Layout::left_to_right(), |ui| {
                    ui.add(Label::new("📓").text_style(egui::TextStyle::Heading));
                });
                // controls
                ui.with_layout(Layout::right_to_left(), |ui| {
                    let close_btn = ui.add(Button::new("❌").text_style(egui::TextStyle::Body));
                    if close_btn.clicked() {
                        frame.quit();
                    }
                    let refresh_btn = ui.add(Button::new("🔄").text_style(egui::TextStyle::Body));
                    let theme_btn = ui.add(Button::new({
                        if self.config.dark_mode {
                            "🌞"
                        } else {
                            "🌙"
                        }
                    }).text_style(egui::TextStyle::Body));
                    if theme_btn.clicked() {
                        self.config.dark_mode = !self.config.dark_mode;
                    }
                });
            });
            ui.add_space(10.);
        });
    }
}
