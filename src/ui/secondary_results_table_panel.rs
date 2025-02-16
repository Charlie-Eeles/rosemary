use crate::app::Rosemary;
use crate::app::ROSEMARY_SORT_COL_STR;
use egui::Ui;
use egui_extras::TableBuilder;

use crate::postgres::CellValue;
use egui_extras::Column as eguiColumn;

// TODO: This is a temporary solution, should logically be merged into the primary editor panel
pub fn show_secondary_results_table_panel(ui: &mut Ui, app: &mut Rosemary) {
    egui::ScrollArea::both().show(ui, |ui| {
        let mut table = TableBuilder::new(ui)
            .striped(true)
            .resizable(true)
            .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
            .min_scrolled_height(0.0);

        for column_name in &app.secondary_res_columns {
            if column_name == ROSEMARY_SORT_COL_STR {
                continue;
            }
            table = table.column(eguiColumn::auto());
        }

        table
            .header(20.0, |mut header| {
                for column_name in &app.secondary_res_columns {
                    if column_name == ROSEMARY_SORT_COL_STR {
                        continue;
                    }
                    header.col(|ui| {
                        egui::Sides::new().show(
                            ui,
                            |ui| {
                                ui.strong(column_name);
                            },
                            |ui| {
                                if ui
                                    .button(if app.secondary_reversed && &app.secondary_sort_by_col == column_name {
                                        "⬆"
                                    } else if &app.secondary_sort_by_col == column_name {
                                        "⬇"
                                    } else {
                                        "⬆⬇"
                                    })
                                    .clicked()
                                {
                                    if &app.secondary_sort_by_col != column_name {
                                        app.secondary_sort_by_col = String::from(column_name);
                                        app.secondary_reversed = false;
                                    } else if !app.secondary_reversed {
                                        app.secondary_reversed = true;
                                    } else if app.secondary_reversed {
                                        app.secondary_reversed = false;
                                        app.secondary_sort_by_col = String::from(ROSEMARY_SORT_COL_STR);
                                    };

                                    if let Some(id_index) =
                                        app.secondary_res_columns.iter().position(|c| c == &app.secondary_sort_by_col)
                                    {
                                        // TODO: simplify this logic
                                        app.secondary_parsed_res_rows.sort_by(|a, b| {
                                            let a_key = match &a[id_index] {
                                                CellValue::SmallInt(v) => {
                                                    (0, Some(*v as i64), None::<&String>)
                                                }
                                                CellValue::MedInt(v) => (0, Some(*v as i64), None),
                                                CellValue::BigInt(v) => (0, Some(*v), None),
                                                CellValue::Text(v) => (1, None, Some(v)),
                                                _ => (2, None, None),
                                            };

                                            let b_key = match &b[id_index] {
                                                CellValue::SmallInt(v) => {
                                                    (0, Some(*v as i64), None)
                                                }
                                                CellValue::MedInt(v) => (0, Some(*v as i64), None),
                                                CellValue::BigInt(v) => (0, Some(*v), None),
                                                CellValue::Text(v) => (1, None, Some(v)),
                                                _ => (2, None, None),
                                            };

                                            if app.secondary_reversed {
                                                b_key.cmp(&a_key)
                                            } else {
                                                a_key.cmp(&b_key)
                                            }
                                        });
                                    }
                                }
                            },
                        );
                    });
                }
            })
            .body(|body| {
                let text_height = 20.0;

                let start_index = app.secondary_current_page * app.secondary_rows_per_page;
                let end_index = (start_index + app.secondary_rows_per_page).min(app.secondary_parsed_res_rows.len());
                let total_rows = if end_index > 0 {
                    end_index - start_index
                } else {
                    0
                };

                body.rows(text_height, total_rows, |mut row| {
                    if let Some(row_data) = app.secondary_parsed_res_rows.get(start_index + row.index()) {
                        for cell in row_data.iter().take(app.secondary_res_columns.len() - 1) {
                            row.col(|ui| {
                                let cell_content = match cell {
                                    CellValue::Text(val) => val.clone(),
                                    CellValue::SmallInt(val) => val.to_string(),
                                    CellValue::MedInt(val) => val.to_string(),
                                    CellValue::BigInt(val) => val.to_string(),
                                    CellValue::SmallFloat(val) => val.to_string(),
                                    CellValue::BigFloat(val) => val.to_string(),
                                    CellValue::Null => "NULL".to_string(),
                                    CellValue::Unsupported => "Unsupported".to_string(),
                                    CellValue::Uuid(val) => val.to_string(),
                                    CellValue::BigDecimal(val) => val.to_string(),
                                };
                                ui.label(cell_content);
                            });
                        }
                    }
                });
            });
    });
}
