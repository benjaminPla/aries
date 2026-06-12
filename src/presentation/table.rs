use eframe::egui;
pub use egui_extras::Column;
use egui_extras::TableBuilder;

use crate::theme::{colors, sizes};

pub fn builder(ui: &mut egui::Ui) -> TableBuilder<'_> {
    TableBuilder::new(ui)
        .striped(true)
        .resizable(false)
        .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
}

pub fn head(ui: &mut egui::Ui, label: &str) {
    ui.painter().rect_filled(ui.max_rect(), egui::CornerRadius::ZERO, colors::BLACK);
    ui.label(
        egui::RichText::new(label)
            .size(sizes::FONT_SIZE_NORMAL)
            .color(colors::LIGHT_GRAY)
            .strong(),
    );
}

pub fn row_height()    -> f32 { sizes::TABLE_ROW_HEIGHT_NORMAL }
pub fn header_height() -> f32 { sizes::TABLE_ROW_HEIGHT_NORMAL }
