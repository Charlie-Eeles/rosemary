use crate::app::format_sql;
use crate::app::Rosemary;
use egui::{Layout, TextEdit, TextStyle, Ui};

pub fn show_editor_panel(
    ui: &mut Ui,
    app: &mut Rosemary,
    should_execute: &mut bool,
    should_execute_secondary: &mut bool,
) {
    let theme = egui_extras::syntax_highlighting::CodeTheme::from_memory(ui.ctx(), ui.style());

    let mut layouter = |ui: &egui::Ui, string: &str, wrap_width: f32| {
        let mut layout_job = egui_extras::syntax_highlighting::highlight(
            ui.ctx(),
            ui.style(),
            &theme,
            string,
            "sql".into(),
        );
        layout_job.wrap.max_width = wrap_width;
        ui.fonts(|f| f.layout_job(layout_job))
    };

    let available_height = ui.ctx().available_rect().height();
    let max_height = if app.show_table_list {
        available_height * 0.65
    } else {
        available_height * 0.95
    };
    egui::ScrollArea::vertical()
        .id_salt("code_editor")
        .max_height(max_height)
        .show(ui, |ui| {
            ui.add(
                TextEdit::multiline(&mut app.code)
                    .font(TextStyle::Monospace)
                    .desired_rows(60)
                    .code_editor()
                    .desired_width(f32::INFINITY)
                    .layouter(&mut layouter),
            );
        });

    ui.with_layout(Layout::right_to_left(egui::Align::TOP), |ui| {
        if ui.add(egui::Button::new("Execute")).clicked() {
            if ui.ctx().input(|i| i.modifiers.shift) {
                *should_execute_secondary = true;
                app.split_results_table = true;
            } else {
                *should_execute = true;
            }
        }

        if ui.add(egui::Button::new("Format")).clicked() {
            app.code = format_sql(&app.code);
        }

        if app.show_table_list {
            if ui.add(egui::Button::new("⬇  Hide tables")).clicked() {
                app.show_table_list = false;
            }
        } else {
            if ui.add(egui::Button::new("⬆  Show tables")).clicked() {
                app.show_table_list = true;
            }
        }

        if app.split_results_table {
            if ui.add(egui::Button::new("Merge")).clicked() {
                app.split_results_table = false;
            }
        } else {
            if ui.add(egui::Button::new("Split")).clicked() {
                app.split_results_table = true;
            }
        }

        ui.checkbox(&mut app.table_queries_are_additive, "Additive queries");
    });
}
