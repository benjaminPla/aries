use std::sync::Arc;

use eframe::egui;

use crate::application::teacher::{
    create::{TeacherCreateInput, TeacherCreateUseCase},
    update::{TeacherUpdateInput, TeacherUpdateUseCase},
};
use crate::domain::teacher::repository::TeacherRepo;
use crate::presentation::{push_error, push_success, Notifications};
use crate::theme::{colors, sizes};

use super::{clear_form, TeachersState};

pub fn show(ctx: &egui::Context, repo: &Arc<dyn TeacherRepo>, state: &mut TeachersState, notifs: &mut Notifications) {
    if !state.show_modal { return; }

    let is_edit = state.editing_id.is_some();
    let title   = if is_edit { "Editar Profesor" } else { "Nuevo Profesor" };

    egui::Window::new(title)
        .collapsible(false)
        .resizable(false)
        .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
        .frame(egui::Frame::new()
            .fill(colors::BLACK)
            .stroke(egui::Stroke::new(sizes::STROKE_SMALL, colors::WHITE))
            .inner_margin(egui::Margin::same(sizes::MARGIN_NORMAL))
        )
        .show(ctx, |ui| {
            ui.with_layout(egui::Layout::top_down_justified(egui::Align::LEFT), |ui| {
                ui.label(egui::RichText::new("Nombre").color(colors::LIGHT_GRAY).size(sizes::FONT_SIZE_NORMAL));
                ui.add_sized([ui.available_width(), 0.0], egui::TextEdit::singleline(&mut state.first_name));
                ui.add_space(sizes::SPACING_SMALL);

                ui.label(egui::RichText::new("Apellido").color(colors::LIGHT_GRAY).size(sizes::FONT_SIZE_NORMAL));
                ui.add_sized([ui.available_width(), 0.0], egui::TextEdit::singleline(&mut state.last_name));
                ui.add_space(sizes::SPACING_SMALL);

                ui.label(egui::RichText::new("Email").color(colors::LIGHT_GRAY).size(sizes::FONT_SIZE_NORMAL));
                ui.add_sized([ui.available_width(), 0.0], egui::TextEdit::singleline(&mut state.email));
                ui.add_space(sizes::SPACING_SMALL);

                ui.label(egui::RichText::new("Teléfono").color(colors::LIGHT_GRAY).size(sizes::FONT_SIZE_NORMAL));
                ui.add_sized([ui.available_width(), 0.0], egui::TextEdit::singleline(&mut state.phone));
                ui.add_space(sizes::SPACING_SMALL);

                ui.label(egui::RichText::new("Notas").color(colors::LIGHT_GRAY).size(sizes::FONT_SIZE_NORMAL));
                ui.add_sized([ui.available_width(), 0.0], egui::TextEdit::multiline(&mut state.notes).desired_rows(3));

                if is_edit {
                    ui.add_space(sizes::SPACING_SMALL);
                    ui.label(egui::RichText::new(format!("Creado: {}", state.created_at)).color(colors::LIGHT_GRAY).size(sizes::FONT_SIZE_NORMAL));
                    ui.label(egui::RichText::new(format!("Editado: {}", state.updated_at)).color(colors::LIGHT_GRAY).size(sizes::FONT_SIZE_NORMAL));
                }

                ui.add_space(sizes::SPACING_NORMAL);
                ui.horizontal(|ui| {
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button("Guardar").clicked() {
                            let notes = if state.notes.trim().is_empty() { None } else { Some(state.notes.clone()) };
                            let result = if is_edit {
                                TeacherUpdateUseCase::new(Arc::clone(repo)).execute(TeacherUpdateInput {
                                    id:         state.editing_id.unwrap(),
                                    email:      state.email.clone(),
                                    first_name: state.first_name.clone(),
                                    last_name:  state.last_name.clone(),
                                    notes,
                                    phone:      state.phone.clone(),
                                })
                            } else {
                                TeacherCreateUseCase::new(Arc::clone(repo)).execute(TeacherCreateInput {
                                    email:      state.email.clone(),
                                    first_name: state.first_name.clone(),
                                    last_name:  state.last_name.clone(),
                                    notes,
                                    phone:      state.phone.clone(),
                                })
                            };
                            match result {
                                Ok(_) => {
                                    push_success(notifs, "Profesor guardado");
                                    state.needs_reload = true;
                                    clear_form(state);
                                }
                                Err(e) => push_error(notifs, e.to_string()),
                            }
                        }
                        if ui.button("Cancelar").clicked() {
                            clear_form(state);
                        }
                    });
                });
            });
        });
}
