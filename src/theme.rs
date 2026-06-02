use eframe::egui::{
    self, Color32, CornerRadius, FontData, FontDefinitions, FontFamily, FontId,
    Margin, Stroke, TextStyle, Visuals,
};
use std::sync::Arc;

// ── Color tokens ─────────────────────────────────────────────────────────────
// Single source of truth. Update here → updates everywhere.

pub mod colors {
    use eframe::egui::Color32;

    // Backgrounds
    pub const BACKGROUND: Color32 = Color32::from_rgb(245, 239, 231); // warm cream
    pub const SURFACE:    Color32 = Color32::from_rgb(255, 252, 248); // near-white — inputs, cards
    pub const SIDEBAR:    Color32 = Color32::from_rgb(234, 224, 210); // warm taupe — left nav

    // Brand
    pub const PRIMARY:       Color32 = Color32::from_rgb(122, 168, 138); // sage green
    pub const PRIMARY_LIGHT: Color32 = Color32::from_rgb(188, 213, 197); // pale sage — hover tint
    pub const ACCENT:        Color32 = Color32::from_rgb(168,  42,  63); // deep crimson

    // Typography
    pub const TEXT:           Color32 = Color32::from_rgb( 42,  28,  18); // warm dark brown
    pub const TEXT_SECONDARY: Color32 = Color32::from_rgb(108,  82,  66); // medium brown
    pub const TEXT_MUTED:     Color32 = Color32::from_rgb(162, 138, 122); // light warm brown

    // Structural
    pub const BORDER:         Color32 = Color32::from_rgb(210, 193, 176); // warm beige
    pub const STRIPE:         Color32 = Color32::from_rgb(239, 231, 220); // subtle grid row tint

    // Semantic
    pub const SUCCESS:        Color32 = Color32::from_rgb( 52, 130,  74);
    pub const ERROR:          Color32 = Color32::from_rgb(168,  42,  63); // = ACCENT
    pub const WARNING:        Color32 = Color32::from_rgb(192, 118,  18);

    // Notification backgrounds
    pub const NOTIF_SUCCESS:  Color32 = Color32::from_rgb( 42, 114,  63);
    pub const NOTIF_ERROR:    Color32 = Color32::from_rgb(148,  33,  52);
    pub const NOTIF_WARNING:  Color32 = Color32::from_rgb(172, 104,  14);

    pub const WHITE: Color32 = Color32::WHITE;
}

// ── Spacing tokens ────────────────────────────────────────────────────────────

pub mod spacing {
    pub const PANEL_MARGIN: i8 = 10;
    pub const CORNER:       u8 =  5; // CornerRadius::same takes u8
    pub const BTN_PAD_X:   f32 = 12.0;
    pub const BTN_PAD_Y:   f32 =  5.0;
    pub const ITEM_SPACING: f32 =  6.0;
}

// ── Theme entry point ─────────────────────────────────────────────────────────

pub fn apply(ctx: &egui::Context) {
    ctx.set_fonts(load_fonts());

    let mut style = (*ctx.global_style()).clone();

    // Text styles
    style.text_styles = [
        (TextStyle::Heading,  FontId::new(18.0, FontFamily::Name("Nunito-Bold".into()))),
        (TextStyle::Body,     FontId::new(14.0, FontFamily::Proportional)),
        (TextStyle::Button,   FontId::new(14.0, FontFamily::Proportional)),
        (TextStyle::Small,    FontId::new(11.5, FontFamily::Proportional)),
        (TextStyle::Monospace, FontId::new(13.0, FontFamily::Monospace)),
    ].into();

    style.spacing.item_spacing   = egui::vec2(spacing::ITEM_SPACING, spacing::ITEM_SPACING);
    style.spacing.button_padding = egui::vec2(spacing::BTN_PAD_X, spacing::BTN_PAD_Y);
    style.spacing.menu_margin    = Margin::same(spacing::PANEL_MARGIN);
    style.spacing.window_margin  = Margin::same(spacing::PANEL_MARGIN);

    ctx.set_global_style(style);

    // Visuals (light mode)
    let cr = CornerRadius::same(spacing::CORNER);
    let mut v = Visuals::light();

    v.panel_fill               = colors::BACKGROUND;
    v.window_fill              = colors::SURFACE;
    v.faint_bg_color           = colors::STRIPE;
    v.extreme_bg_color         = colors::SURFACE;
    v.code_bg_color            = colors::STRIPE;

    v.window_corner_radius     = cr;
    v.menu_corner_radius       = cr;
    v.window_stroke            = Stroke::new(1.0, colors::BORDER);

    v.warn_fg_color            = colors::WARNING;
    v.error_fg_color           = colors::ERROR;
    v.hyperlink_color          = colors::PRIMARY;

    v.selection.bg_fill        = colors::PRIMARY_LIGHT;
    v.selection.stroke         = Stroke::new(1.0, colors::PRIMARY);

    // Widget states
    v.widgets.noninteractive.corner_radius = cr;
    v.widgets.noninteractive.bg_fill       = colors::BACKGROUND;
    v.widgets.noninteractive.weak_bg_fill  = colors::STRIPE;
    v.widgets.noninteractive.fg_stroke     = Stroke::new(1.0, colors::TEXT_SECONDARY);
    v.widgets.noninteractive.bg_stroke     = Stroke::new(1.0, colors::BORDER);

    v.widgets.inactive.corner_radius       = cr;
    v.widgets.inactive.bg_fill            = colors::SURFACE;
    v.widgets.inactive.weak_bg_fill       = colors::STRIPE;
    v.widgets.inactive.fg_stroke          = Stroke::new(1.0, colors::TEXT);
    v.widgets.inactive.bg_stroke          = Stroke::new(1.0, colors::BORDER);

    v.widgets.hovered.corner_radius        = cr;
    v.widgets.hovered.bg_fill             = colors::PRIMARY_LIGHT;
    v.widgets.hovered.weak_bg_fill        = colors::PRIMARY_LIGHT;
    v.widgets.hovered.fg_stroke           = Stroke::new(1.5, colors::TEXT);
    v.widgets.hovered.bg_stroke           = Stroke::new(1.5, colors::PRIMARY);

    v.widgets.active.corner_radius         = cr;
    v.widgets.active.bg_fill              = colors::PRIMARY;
    v.widgets.active.weak_bg_fill         = colors::PRIMARY;
    v.widgets.active.fg_stroke            = Stroke::new(2.0, colors::WHITE);
    v.widgets.active.bg_stroke            = Stroke::new(1.5, colors::PRIMARY);

    v.widgets.open.corner_radius           = cr;
    v.widgets.open.bg_fill                = colors::PRIMARY_LIGHT;
    v.widgets.open.weak_bg_fill           = colors::PRIMARY_LIGHT;
    v.widgets.open.fg_stroke              = Stroke::new(1.0, colors::TEXT);
    v.widgets.open.bg_stroke              = Stroke::new(1.0, colors::PRIMARY);

    ctx.set_visuals(v);
}

// ── Font loading ──────────────────────────────────────────────────────────────

fn load_fonts() -> FontDefinitions {
    let mut fonts = FontDefinitions::default();

    fonts.font_data.insert(
        "Nunito".into(),
        Arc::new(FontData::from_static(include_bytes!("../assets/fonts/Nunito-Regular.ttf"))),
    );
    fonts.font_data.insert(
        "Nunito-Bold".into(),
        Arc::new(FontData::from_static(include_bytes!("../assets/fonts/Nunito-Bold.ttf"))),
    );
    fonts.font_data.insert(
        "Nunito-Italic".into(),
        Arc::new(FontData::from_static(include_bytes!("../assets/fonts/Nunito-Italic.ttf"))),
    );

    fonts.families
        .entry(FontFamily::Proportional)
        .or_default()
        .insert(0, "Nunito".into());

    fonts.families.insert(
        FontFamily::Name("Nunito-Bold".into()),
        vec!["Nunito-Bold".into(), "Nunito".into()],
    );

    fonts
}

// ── Convenience ───────────────────────────────────────────────────────────────

/// Borderless panel frame with brand fill and consistent inner margin.
pub fn panel_frame(fill: Color32) -> egui::Frame {
    egui::Frame::new()
        .fill(fill)
        .inner_margin(Margin::same(spacing::PANEL_MARGIN))
}
