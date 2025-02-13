use egui::{Ui, Layout, TextEdit, TextStyle};
use crate::app::format_sql;
use crate::app::Rosemary;

pub fn show_editor_panel(ui: &mut Ui, app: &mut Rosemary, should_execute: &mut bool) {
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

    egui::ScrollArea::vertical()
        .id_salt("code_editor")
        .max_height(550.0)
        .show(ui, |ui| {
            ui.add(
                TextEdit::multiline(&mut app.code)
                    .font(TextStyle::Monospace)
                    .desired_rows(55)
                    .code_editor()
                    .desired_width(f32::INFINITY)
                    .layouter(&mut layouter),
            );
        });

    ui.with_layout(Layout::right_to_left(egui::Align::TOP), |ui| {
        if ui.add(egui::Button::new("Execute")).clicked() {
            *should_execute = true;
        }

        if ui.add(egui::Button::new("Format")).clicked() {
            app.code = format_sql(&app.code);
        }
    });


}
