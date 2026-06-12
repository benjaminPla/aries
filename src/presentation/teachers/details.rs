use eframe::egui;

use crate::presentation::fmt_dt;
use crate::theme::{colors, sizes};

use super::{Mode, TeachersState};

pub fn show(ui: &mut egui::Ui, state: &mut TeachersState) {
    let Some(teacher) = state.viewing_id
        .and_then(|id| state.teachers.iter().find(|t| t.id == id))
        .cloned()
    else {
        state.mode = Mode::List;
        return;
    };

    // ── Navigation ────────────────────────────────────────────────────────────

    if ui.button("<- Volver").clicked() {
        state.viewing_id = None;
        state.mode       = Mode::List;
    }
    ui.separator();

    // ── Information ──────────────────────────────────────────────────────────

    ui.label(egui::RichText::new("Información").size(sizes::FONT_SIZE_BIG).color(colors::LIGHT_GRAY).strong());

    egui::Grid::new("teacher_details").num_columns(2).spacing([sizes::SPACING_NORMAL, sizes::SPACING_EXTRA_SMAL]).show(ui, |ui| {
        ui.label(egui::RichText::new("Nombre").color(colors::LIGHT_GRAY));
        ui.label(&teacher.first_name);
        ui.end_row();

        ui.label(egui::RichText::new("Apellido").color(colors::LIGHT_GRAY));
        ui.label(&teacher.last_name);
        ui.end_row();

        ui.label(egui::RichText::new("Email").color(colors::LIGHT_GRAY));
        ui.label(&teacher.email);
        ui.end_row();
       
        ui.label(egui::RichText::new("Teléfono").color(colors::LIGHT_GRAY));
        ui.label(&teacher.phone);
        ui.end_row();

        if let Some(n) = &teacher.notes {
            ui.label(egui::RichText::new("Notas").color(colors::LIGHT_GRAY));
            ui.label(n.as_str());
            ui.end_row();
        }

        ui.label(egui::RichText::new("Creado").color(colors::LIGHT_GRAY));
        ui.label(fmt_dt(teacher.created_at));
        ui.end_row();

        ui.label(egui::RichText::new("Editado").color(colors::LIGHT_GRAY));
        ui.label(fmt_dt(teacher.updated_at));
        ui.end_row();
    });
}
