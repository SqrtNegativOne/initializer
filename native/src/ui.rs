use eframe::egui::{
    self, Align, Color32, FontData, FontDefinitions, FontFamily, FontId, Stroke, Vec2,
};

pub const GREEN: Color32 = Color32::from_rgb(0x38, 0x66, 0x41);
pub const YELLOW_GREEN: Color32 = Color32::from_rgb(0xa7, 0xc9, 0x57);
pub const CREAM: Color32 = Color32::from_rgb(0xff, 0xff, 0xff);

const AXIA_REGULAR: &[u8] = include_bytes!("../assets/axia-font-family/Axia-Regular-TRIAL.otf");

pub fn configure_fonts(ctx: &egui::Context) {
    let mut fonts = FontDefinitions::default();
    fonts
        .font_data
        .insert("axia".to_owned(), FontData::from_static(AXIA_REGULAR));

    if let Some(family) = fonts.families.get_mut(&FontFamily::Proportional) {
        family.insert(0, "axia".to_owned());
    }
    if let Some(family) = fonts.families.get_mut(&FontFamily::Monospace) {
        family.push("axia".to_owned());
    }

    ctx.set_fonts(fonts);
}

pub fn configure_style(ctx: &egui::Context) {
    let mut style = (*ctx.style()).clone();
    style.visuals = egui::Visuals::light();
    style.visuals.panel_fill = CREAM;
    style.visuals.window_fill = CREAM;
    style.spacing.item_spacing = Vec2::new(8.0, 8.0);
    ctx.set_style(style);
}

pub fn style_launcher(ui: &mut egui::Ui) {
    let visuals = &mut ui.style_mut().visuals;
    let widgets = &mut visuals.widgets;

    widgets.inactive.bg_fill = Color32::TRANSPARENT;
    widgets.inactive.weak_bg_fill = Color32::TRANSPARENT;
    widgets.inactive.bg_stroke = Stroke::new(2.0_f32, GREEN);
    widgets.inactive.fg_stroke = Stroke::new(1.5_f32, GREEN);
    widgets.inactive.rounding = egui::Rounding::ZERO;

    widgets.hovered.bg_fill = GREEN;
    widgets.hovered.weak_bg_fill = GREEN;
    widgets.hovered.bg_stroke = Stroke::new(2.0_f32, GREEN);
    widgets.hovered.fg_stroke = Stroke::new(1.5_f32, CREAM);
    widgets.hovered.rounding = egui::Rounding::ZERO;

    widgets.active.bg_fill = GREEN;
    widgets.active.weak_bg_fill = GREEN;
    widgets.active.bg_stroke = Stroke::new(2.0_f32, GREEN);
    widgets.active.fg_stroke = Stroke::new(1.5_f32, CREAM);
    widgets.active.rounding = egui::Rounding::ZERO;
}

pub fn style_quick_job(ui: &mut egui::Ui) {
    let visuals = &mut ui.style_mut().visuals;
    visuals.override_text_color = Some(GREEN);
    visuals.extreme_bg_color = Color32::TRANSPARENT;
    visuals.selection.bg_fill = GREEN.gamma_multiply(0.35);

    let widgets = &mut visuals.widgets;
    widgets.inactive.bg_fill = Color32::TRANSPARENT;
    widgets.inactive.weak_bg_fill = Color32::TRANSPARENT;
    widgets.inactive.bg_stroke = Stroke::NONE;
    widgets.hovered.bg_fill = Color32::TRANSPARENT;
    widgets.hovered.weak_bg_fill = Color32::TRANSPARENT;
    widgets.hovered.bg_stroke = Stroke::NONE;
    widgets.active.bg_fill = Color32::TRANSPARENT;
    widgets.active.weak_bg_fill = Color32::TRANSPARENT;
    widgets.active.bg_stroke = Stroke::NONE;
}

pub fn make_job(
    text: &str,
    size: f32,
    color: Color32,
    strikethrough: bool,
    max_width: f32,
) -> egui::text::LayoutJob {
    let mut job = egui::text::LayoutJob {
        halign: Align::Center,
        wrap: egui::text::TextWrapping {
            max_width,
            ..Default::default()
        },
        ..Default::default()
    };
    job.append(
        text,
        0.0,
        egui::TextFormat {
            font_id: FontId::proportional(size),
            color,
            strikethrough: if strikethrough {
                Stroke::new(1.5_f32, color)
            } else {
                Stroke::NONE
            },
            ..Default::default()
        },
    );
    job
}
