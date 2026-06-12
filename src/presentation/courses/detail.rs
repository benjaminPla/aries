use std::sync::{Arc, Mutex};

use chrono::{Duration, NaiveDate};
use eframe::egui;
use postgres::Client;
use uuid::Uuid;

use crate::{
    application::{
        course_period::{
            create::{CoursePeriodCreateInput, CoursePeriodCreateUseCase},
            delete::CoursePeriodDeleteUseCase,
            get_by_course::CoursePeriodGetByCourseUseCase,
        },
        teacher::get_all::TeacherGetAllUseCase,
    },
    presentation::table::{self, Column},
    presentation::{confirm_delete_modal, push_error, push_success, section_header, Notifications},
    theme::colors::DARK_GRAY
};

use super::{format_price, make_course_period_repo, CoursesState, Mode};

pub fn show(
    ui:     &mut egui::Ui,
    client: &Arc<Mutex<Client>>,
    state:  &mut CoursesState,
    notifs: &mut Notifications,
) {
    let course = match &state.selected_course {
        Some(c) => c.clone(),
        None => {
            state.mode = Mode::List;
            return;
        }
    };

    if state.teachers.is_empty() {
        if let Ok(ts) = TeacherGetAllUseCase::new(super::make_teacher_repo(client)).execute() {
            state.teachers = ts;
        }
    }

    if state.needs_reload_periods {
        state.needs_reload_periods = false;
        match CoursePeriodGetByCourseUseCase::new(make_course_period_repo(client)).execute(course.id) {
            Ok(periods) => state.periods = periods,
            Err(e)      => push_error(notifs, e.to_string()),
        }
    }

    // ── Navigation ────────────────────────────────────────────────────────────
    if ui.button("<- Volver").clicked() {
        state.mode             = Mode::List;
        state.selected_course  = None;
        state.periods          = Vec::new();
        state.show_period_form = false;
        return;
    }
    ui.separator();

    // ── Information ──────────────────────────────────────────────────────────
    section_header(ui, "Información");
    egui::Grid::new("course_details").num_columns(2).show(ui, |ui| {
        let teacher_name = state.teachers.iter()
            .find(|t| t.id == course.teacher_id)
            .map(|t| format!("{} {}", t.first_name, t.last_name))
            .unwrap_or_else(|| course.teacher_id.to_string());

        ui.label(egui::RichText::new("Nombre").color(DARK_GRAY));
        ui.label(&course.name);
        ui.end_row();

        ui.label(egui::RichText::new("Profesor").color(DARK_GRAY));
        ui.label(teacher_name);
        ui.end_row();

        ui.label(egui::RichText::new("Grupo").color(DARK_GRAY));
        ui.label(course.age_group.label());
        ui.end_row();

        ui.label(egui::RichText::new("Capacidad").color(DARK_GRAY));
        ui.label(course.capacity.to_string());
        ui.end_row();

        ui.label(egui::RichText::new("Precio menusal").color(DARK_GRAY));
        ui.label(format_price(course.month_price_cents));
        ui.end_row();

        ui.label(egui::RichText::new("Precio clase").color(DARK_GRAY));
        ui.label(format_price(course.class_price_cents));
        ui.end_row();

        if let Some(n) = &course.notes {
            ui.label(egui::RichText::new("Notas").color(DARK_GRAY));
            ui.label(n);
            ui.end_row();
        }

        ui.label(egui::RichText::new("Creado").color(DARK_GRAY));
        ui.label(crate::presentation::fmt_dt(course.created_at));
        ui.end_row();

        ui.label(egui::RichText::new("Editado").color(DARK_GRAY));
        ui.label(crate::presentation::fmt_dt(course.updated_at));
        ui.end_row();
    });
    ui.add_space(4.0);
    ui.separator();

    // ── Periods ──────────────────────────────────────────────────────────────
    ui.horizontal(|ui| {
        ui.label(egui::RichText::new("Períodos").size(crate::theme::sizes::FONT_SIZE_SMALL).strong());
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if ui.button("+ Nuevo período").clicked() {
                state.show_period_form = true;
            }
        });
    });

    // ── New period modal ──────────────────────────────────────────────────────
    if state.show_period_form {
        egui::Window::new("Nuevo período")
            .collapsible(false)
            .resizable(false)
            .auto_sized()
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .show(ui.ctx(), |ui| {
                ui.label("Año");
                egui::ComboBox::from_id_salt("period_year")
                    .width(200.0)
                    .selected_text(state.period_year.to_string())
                    .show_ui(ui, |ui| {
                        for y in 2024..=2030 {
                            ui.selectable_value(&mut state.period_year, y, y.to_string());
                        }
                    });
                ui.add_space(4.0);
                ui.label("Mes");
                egui::ComboBox::from_id_salt("period_month")
                    .width(200.0)
                    .selected_text(MONTHS[(state.period_month - 1) as usize])
                    .show_ui(ui, |ui| {
                        for (i, name) in MONTHS.iter().enumerate() {
                            ui.selectable_value(&mut state.period_month, (i + 1) as u32, *name);
                        }
                    });
                ui.add_space(8.0);
                ui.horizontal(|ui| {
                    if ui.button("Cancelar").clicked() {
                        state.show_period_form = false;
                    }
                    if ui.button("Guardar").clicked() {
                        let y = state.period_year;
                        let m = state.period_month;
                        let start_date = NaiveDate::from_ymd_opt(y, m, 1).unwrap();
                        let end_date = if m == 12 {
                            NaiveDate::from_ymd_opt(y + 1, 1, 1).unwrap() - Duration::days(1)
                        } else {
                            NaiveDate::from_ymd_opt(y, m + 1, 1).unwrap() - Duration::days(1)
                        };
                        match CoursePeriodCreateUseCase::new(make_course_period_repo(client)).execute(
                            CoursePeriodCreateInput { course_id: course.id, start_date, end_date },
                        ) {
                            Ok(_) => {
                                push_success(notifs, "Período creado");
                                state.show_period_form     = false;
                                state.needs_reload_periods = true;
                            }
                            Err(e) => push_error(notifs, e.to_string()),
                        }
                    }
                });
            });
    }

    // ── Periods table ─────────────────────────────────────────────────────────
    let mut delete_id: Option<Uuid> = None;

    table::builder(ui)
        .column(Column::remainder())
        .column(Column::auto())
        .column(Column::auto())
        .column(Column::auto())
        .column(Column::auto())
        .column(Column::auto())
        .header(table::header_height(), |mut h| {
            h.col(|ui| table::head(ui, "Etiqueta"));
            h.col(|ui| table::head(ui, "Inicio"));
            h.col(|ui| table::head(ui, "Fin"));
            h.col(|ui| table::head(ui, "Inscritos"));
            h.col(|ui| table::head(ui, "Estado"));
            h.col(|ui| table::head(ui, "Acciones"));
        })
        .body(|mut body| {
            for p in &state.periods {
                body.row(table::row_height(), |mut row| {
                    row.col(|ui| { ui.label(&p.label); });
                    row.col(|ui| { ui.label(p.start_date.format("%d/%m/%Y").to_string()); });
                    row.col(|ui| { ui.label(p.end_date.format("%d/%m/%Y").to_string()); });
                    row.col(|ui| { ui.label(p.enrolled.to_string()); });
                    row.col(|ui| {
                        let today = chrono::Local::now().date_naive();
                        if p.start_date > today {
                            ui.colored_label(crate::theme::colors::YELLOW, "Futuro");
                        } else if p.end_date >= today {
                            ui.colored_label(crate::theme::colors::GREEN, "Activo");
                        } else {
                            ui.colored_label(crate::theme::colors::DARK_GRAY, "Finalizado");
                        }
                    });
                    row.col(|ui| {
                        if ui.small_button("x").clicked() { delete_id = Some(p.id); }
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

const MONTHS: [&str; 12] = [
    "Enero", "Febrero", "Marzo", "Abril", "Mayo", "Junio",
    "Julio", "Agosto", "Septiembre", "Octubre", "Noviembre", "Diciembre",
];
