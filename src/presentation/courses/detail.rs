use std::sync::{Arc, Mutex};

use eframe::egui;
use postgres::Client;
use uuid::Uuid;

use crate::{
    application::course_period::{
        create::{CoursePeriodCreateInput, CoursePeriodCreateUseCase},
        delete::CoursePeriodDeleteUseCase,
        get_by_course::CoursePeriodGetByCourseUseCase,
    },
    presentation::{confirm_delete_modal, push_error, push_success, Notifications},
    presentation::table::{self, Column},
};

use super::{CoursesState, Mode, format_price, make_course_period_repo};

pub fn show(ui: &mut egui::Ui, client: &Arc<Mutex<Client>>, state: &mut CoursesState, notifs: &mut Notifications) {
    let course = match &state.selected_course {
        Some(c) => c.clone(),
        None    => { state.mode = Mode::List; return; }
    };

    if state.needs_reload_periods {
        match CoursePeriodGetByCourseUseCase::new(make_course_period_repo(client)).execute(course.id) {
            Ok(periods) => { state.periods = periods; state.needs_reload_periods = false; }
            Err(e)      => push_error(notifs, e.to_string()),
        }
    }

    ui.horizontal(|ui| {
        if ui.button("← Cursos").clicked() {
            state.mode                 = Mode::List;
            state.selected_course      = None;
            state.periods              = Vec::new();
            state.show_period_form     = false;
            return;
        }
        ui.heading(format!("{} — {}", course.name, course.age_group.label()));
    });

    egui::Grid::new("course_detail_info").num_columns(2).show(ui, |ui| {
        ui.label("Profesor");       ui.label(&course.teacher_name);                              ui.end_row();
        ui.label("Capacidad");      ui.label(course.capacity.to_string());                       ui.end_row();
        ui.label("Precio mensual"); ui.label(format!("${}", format_price(course.price_cents)));  ui.end_row();
        ui.label("Precio clase");   ui.label(format!("${}", format_price(course.class_price_cents))); ui.end_row();
        if let Some(n) = &course.notes {
            ui.label("Notas"); ui.label(n); ui.end_row();
        }
    });

    ui.separator();

    // ── Period form ──────────────────────────────────────────────────────────
    if state.show_period_form {
        ui.heading("Nuevo período");
        egui::Grid::new("period_form").num_columns(2).show(ui, |ui| {
            ui.label("Etiqueta");
            ui.text_edit_singleline(&mut state.period_label);
            ui.end_row();

            ui.label("Fecha inicio");
            date_picker(ui, "period_start", &mut state.period_start_date);
            ui.end_row();

            ui.label("Fecha fin");
            date_picker(ui, "period_end", &mut state.period_end_date);
            ui.end_row();
        });

        ui.horizontal(|ui| {
            if ui.button("Guardar").clicked() {
                match (&state.period_start_date, &state.period_end_date) {
                    (Some(start), Some(end)) => {
                        let result = CoursePeriodCreateUseCase::new(make_course_period_repo(client))
                            .execute(CoursePeriodCreateInput {
                                course_id:  course.id,
                                label:      state.period_label.clone(),
                                start_date: *start,
                                end_date:   *end,
                            });
                        match result {
                            Ok(_) => {
                                push_success(notifs, "Período creado");
                                state.show_period_form     = false;
                                state.period_label         = String::new();
                                state.period_start_date    = None;
                                state.period_end_date      = None;
                                state.needs_reload_periods = true;
                            }
                            Err(e) => push_error(notifs, e.to_string()),
                        }
                    }
                    _ => push_error(notifs, "Seleccionar fechas de inicio y fin"),
                }
            }
            if ui.button("Cancelar").clicked() {
                state.show_period_form  = false;
                state.period_label      = String::new();
                state.period_start_date = None;
                state.period_end_date   = None;
            }
        });
        ui.separator();
    } else {
        ui.horizontal(|ui| {
            ui.heading("Períodos");
            if ui.button("+ Nuevo período").clicked() {
                state.show_period_form = true;
            }
        });
    }

    // ── Periods table ────────────────────────────────────────────────────────
    let mut delete_id: Option<Uuid> = None;

    table::builder(ui)
        .column(Column::remainder().at_least(100.0))
        .column(Column::exact(90.0))
        .column(Column::exact(90.0))
        .column(Column::exact(70.0))
        .column(Column::exact(80.0))
        .column(Column::auto())
        .header(table::header_height(), |mut h| {
            h.col(|ui| table::head(ui, "Etiqueta"));
            h.col(|ui| table::head(ui, "Inicio"));
            h.col(|ui| table::head(ui, "Fin"));
            h.col(|ui| table::head(ui, "Inscritos"));
            h.col(|ui| table::head(ui, "Estado"));
            h.col(|ui| table::head(ui, ""));
        })
        .body(|mut body| {
            for p in &state.periods {
                body.row(table::row_height(), |mut row| {
                    row.col(|ui| { ui.label(&p.label); });
                    row.col(|ui| { ui.label(p.start_date.format("%d/%m/%Y").to_string()); });
                    row.col(|ui| { ui.label(p.end_date.format("%d/%m/%Y").to_string()); });
                    row.col(|ui| { ui.label(p.enrolled.to_string()); });
                    row.col(|ui| {
                        if p.is_active() {
                            ui.colored_label(crate::theme::colors::SUCCESS, "Activo");
                        } else {
                            ui.colored_label(crate::theme::colors::TEXT_MUTED, "Finalizado");
                        }
                    });
                    row.col(|ui| {
                        if ui.small_button("🗑").clicked() { delete_id = Some(p.id); }
                    });
                });
            }
        });

    if let Some(id) = delete_id { state.confirm_delete_period = Some(id); }

    if let Some(id) = confirm_delete_modal(ui.ctx(), &mut state.confirm_delete_period) {
        match CoursePeriodDeleteUseCase::new(make_course_period_repo(client)).execute(id) {
            Ok(_)  => { push_success(notifs, "Período eliminado"); state.needs_reload_periods = true; }
            Err(e) => push_error(notifs, e.to_string()),
        }
    }
}

// Minimal inline date picker: YYYY-MM-DD text field that parses on change
fn date_picker(ui: &mut egui::Ui, id: &str, date: &mut Option<chrono::NaiveDate>) {
    let mut text = date.map(|d| d.format("%Y-%m-%d").to_string()).unwrap_or_default();
    let response = ui.add(egui::TextEdit::singleline(&mut text).hint_text("YYYY-MM-DD").id(egui::Id::new(id)));
    if response.changed() {
        *date = chrono::NaiveDate::parse_from_str(&text, "%Y-%m-%d").ok();
    }
}
