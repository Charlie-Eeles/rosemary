use crate::app::format_sql;
use crate::app::Rosemary;
use egui::Ui;

pub fn show_tables_panel(ui: &mut Ui, app: &mut Rosemary, should_execute: &mut bool) {
    ui.horizontal(|ui| {
        ui.label("Filter:");
        ui.add(egui::TextEdit::singleline(&mut app.table_filter));
        app.table_filter = app.table_filter.to_lowercase();
        if ui.button("ｘ").clicked() {
            app.table_filter.clear();
        }
    });

    egui::ScrollArea::vertical()
        .id_salt("public_table_list")
        .show(ui, |ui| {
            for table in &app.tables {
                let table_name = table.table_name.as_deref().unwrap_or("NULL");
                if !app.table_filter.trim().is_empty()
                    && !table_name.to_lowercase().contains(&app.table_filter)
                {
                    continue;
                }
                let table_type = table.table_type.as_deref().unwrap_or("NULL");
                let button_label = format!("{table_name} [{table_type}]");
                let button = egui::Button::new(button_label);

                if ui.add_sized([ui.available_width(), 0.0], button).clicked() {
                    if app.table_queries_are_additive {
                        let code = app.code.clone() + &format!("SELECT * FROM {table_name};");
                        app.code = format_sql(&code);
                    } else {
                        app.code = format_sql(&format!("SELECT * FROM {table_name};"));
                    }
                    *should_execute = true;
                }
            }
        });
}
