use std::collections::BTreeMap;

use crate::{app::Rosemary, query_functions::{pg_data::PublicTable, pg_query_handlers::format_sql}};
use egui::Ui;

//TODO: Improve this arg logic
pub fn show_tables_panel(
    ui: &mut Ui,
    app: &mut Rosemary,
    should_execute: &mut bool,
    should_execute_secondary: &mut bool,
    shift_pressed: bool,
) {
    ui.horizontal(|ui| {
        ui.label("Filter:");
        ui.add(egui::TextEdit::singleline(&mut app.table_filter));
        app.table_filter = app.table_filter.to_lowercase();
        if ui.button("ｘ").clicked() {
            app.table_filter.clear();
        }
    });
    let mut schema_table_map = BTreeMap::new();
    for table in &app.tables {
        match &table.table_schema {
            Some(schema) => {
                schema_table_map
                    .entry(schema)
                    .or_insert(Vec::new())
                    .push(table);
            }
            None => {}
        }
    }

    egui::ScrollArea::vertical()
        .id_salt("public_table_list")
        .show(ui, |ui| {
            for (schema, tables) in &schema_table_map {
                ui.push_id(schema, |ui| {
                    let filtered_tables: Vec<&&PublicTable> = if app.table_filter.trim().is_empty() {
                        tables.iter().collect()
                    } else {
                        tables
                            .iter()
                            .filter(|table| {
                                table
                                    .table_name
                                    .as_deref()
                                    .unwrap_or("NULL")
                                    .to_lowercase()
                                    .contains(&app.table_filter)
                            })
                            .collect()
                    };
                    ui.collapsing(
                        format!("{} ( {} )", schema, filtered_tables.len()),
                        |ui| {
                            for table in filtered_tables {
                                let table_name = table.table_name.as_deref().unwrap_or("NULL");
                                let table_schema = table.table_schema.as_deref().unwrap_or("NULL");

                                let table_type = table.table_type.as_deref().unwrap_or("NULL");
                                let button_label = format!("{table_name} [{table_type}]");
                                let button = egui::Button::new(button_label);

                                if ui.add_sized([ui.available_width(), 0.0], button).clicked() {
                                    if app.table_queries_are_additive {
                                        let code = app.code.clone()
                                            + &format!(
                                                "SELECT * FROM {table_schema}.{table_name};"
                                            );
                                        app.code = format_sql(&code);
                                    } else {
                                        app.code = format_sql(&format!(
                                            "SELECT * FROM {table_schema}.{table_name};"
                                        ));
                                    }

                                    if shift_pressed {
                                        if !app.split_results_table {
                                            app.split_results_table = true;
                                        }
                                        *should_execute_secondary = true;
                                    } else {
                                        *should_execute = true;
                                    }
                                }
                            }
                        },
                    );
                });
            }
        });
}
