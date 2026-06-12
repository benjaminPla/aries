use eframe::egui::{
    self, Color32, FontFamily, FontId,
    Margin, Stroke, TextStyle, Theme, Visuals,
};

// ── Colors ───────────────────────────────────────────────────────────────

pub mod colors {
    use eframe::egui::Color32;
    pub const BLACK:       Color32 = Color32::BLACK;
    pub const BLUE:        Color32 = Color32::from_rgb(0x21, 0x96, 0xF3);
    pub const DARK_GRAY:   Color32 = Color32::from_rgb(0x18, 0x18, 0x18);
    pub const GREEN:       Color32 = Color32::from_rgb(0x4C, 0xAF, 0x50);
    pub const LIGHT_GRAY:  Color32 = Color32::from_rgb(0xB0, 0xB0, 0xB0);
    pub const RED:         Color32 = Color32::from_rgb(0xFF, 0x4D, 0x4D);
    pub const TRANSPARENT: Color32 = Color32::TRANSPARENT;
    pub const WHITE:       Color32 = Color32::WHITE;
    pub const YELLOW:      Color32 = Color32::from_rgb(0xFF, 0xC1, 0x07);
}

// ── Sizes ────────────────────────────────────────────────────────────────

pub mod sizes {
    use eframe::egui::CornerRadius;
    pub const CORNER_RADIUS_NONE:      CornerRadius = CornerRadius::ZERO;
    pub const FONT_SIZE_BIG:           f32          = 14.0;
    pub const FONT_SIZE_NORMAL:        f32          = 12.0;
    pub const FONT_SIZE_SMALL:         f32          = 10.0;
    pub const MARGIN_NORMAL:           i8           = 6;
    pub const SPACING_EXTRA_SMAL:      f32          = 2.0;
    pub const SPACING_NORMAL:          f32          = 8.0;
    pub const SPACING_SMALL:           f32          = 4.0;
    pub const STROKE_SMALL:            f32          = 0.3;
    pub const TABLE_ROW_HEIGHT_NORMAL: f32          = 20.0;
}

// ── Theme ────────────────────────────────────────────────────────────────

pub fn apply(ctx: &egui::Context) {
    ctx.set_theme(Theme::Dark);

    let mut fonts = egui::FontDefinitions::default();
    egui_phosphor::add_to_fonts(&mut fonts, egui_phosphor::Variant::Regular);
    ctx.set_fonts(fonts);

    ctx.all_styles_mut(|style| {
        style.text_styles = [
            (TextStyle::Body,    FontId::new(sizes::FONT_SIZE_NORMAL, FontFamily::Proportional)),
            (TextStyle::Button,  FontId::new(sizes::FONT_SIZE_NORMAL, FontFamily::Proportional)),
            (TextStyle::Heading, FontId::new(sizes::FONT_SIZE_BIG,    FontFamily::Proportional)),
            (TextStyle::Small,   FontId::new(sizes::FONT_SIZE_SMALL,  FontFamily::Proportional)),
        ]
        .into();

        style.spacing.button_padding =   egui::vec2(sizes::SPACING_NORMAL, sizes::SPACING_SMALL);
        style.spacing.item_spacing   =   egui::vec2(sizes::SPACING_NORMAL, sizes::SPACING_SMALL);
        style.spacing.menu_margin    = Margin::same(sizes::MARGIN_NORMAL);
        style.spacing.window_margin  = Margin::same(sizes::MARGIN_NORMAL);
    });

    let mut v = Visuals::dark();

    let stroke = Stroke { width: 0.1, color: colors::LIGHT_GRAY };

    v.error_fg_color                       = colors::RED;
    v.extreme_bg_color                     = colors::DARK_GRAY;
    v.faint_bg_color                       = colors::DARK_GRAY;
    v.hyperlink_color                      = colors::BLUE;
    v.menu_corner_radius                   = sizes::CORNER_RADIUS_NONE;
    v.override_text_color                  = Some(colors::WHITE);
    v.panel_fill                           = colors::BLACK;
    v.selection.bg_fill                    = colors::DARK_GRAY;
    v.selection.stroke                     = stroke;
    v.warn_fg_color                        = colors::YELLOW;
    v.widgets.active.bg_fill               = colors::LIGHT_GRAY;
    v.widgets.active.bg_stroke             = stroke;
    v.widgets.active.corner_radius         = sizes::CORNER_RADIUS_NONE;
    v.widgets.active.fg_stroke             = stroke;
    v.widgets.active.weak_bg_fill          = colors::LIGHT_GRAY;
    v.widgets.hovered.bg_fill              = colors::LIGHT_GRAY;
    v.widgets.hovered.bg_stroke            = stroke;
    v.widgets.hovered.corner_radius        = sizes::CORNER_RADIUS_NONE;
    v.widgets.hovered.fg_stroke            = stroke;
    v.widgets.hovered.weak_bg_fill         = colors::DARK_GRAY;
    v.widgets.inactive.bg_fill             = Color32::TRANSPARENT;
    v.widgets.inactive.bg_stroke           = stroke;
    v.widgets.inactive.corner_radius       = sizes::CORNER_RADIUS_NONE;
    v.widgets.inactive.fg_stroke           = stroke;
    v.widgets.inactive.weak_bg_fill        = Color32::TRANSPARENT;
    v.widgets.noninteractive.bg_fill       = colors::TRANSPARENT;
    v.widgets.noninteractive.bg_stroke     = stroke;
    v.widgets.noninteractive.corner_radius = sizes::CORNER_RADIUS_NONE;
    v.widgets.noninteractive.fg_stroke     = stroke;
    v.widgets.noninteractive.fg_stroke.color     = colors::WHITE;
    v.widgets.noninteractive.weak_bg_fill  = colors::TRANSPARENT;
    v.widgets.open.bg_fill                 = colors::LIGHT_GRAY;
    v.widgets.open.bg_stroke               = stroke;
    v.widgets.open.corner_radius           = sizes::CORNER_RADIUS_NONE;
    v.widgets.open.fg_stroke               = stroke;
    v.widgets.open.weak_bg_fill            = colors::LIGHT_GRAY;
    v.window_corner_radius                 = sizes::CORNER_RADIUS_NONE;
    v.window_fill                          = colors::BLACK;
    v.window_stroke                        = Stroke::new(sizes::STROKE_SMALL, colors::BLACK);

    ctx.set_visuals_of(Theme::Dark,  v.clone());
    ctx.set_visuals_of(Theme::Light, v);
}
