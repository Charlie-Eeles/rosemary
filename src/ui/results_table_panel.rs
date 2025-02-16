use crate::app::QueryResultsPanel;
use crate::app::ROSEMARY_SORT_COL_STR;
use egui::Ui;
use egui_extras::TableBuilder;

use crate::postgres::CellValue;
use egui_extras::Column as eguiColumn;

pub fn show_results_table_panel(ui: &mut Ui, app: &mut QueryResultsPanel) {
    egui::ScrollArea::both().show(ui, |ui| {
        let mut table = TableBuilder::new(ui)
            .striped(true)
            .resizable(true)
            .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
            .min_scrolled_height(0.0);

        for column_name in &app.res_columns {
            if column_name == ROSEMARY_SORT_COL_STR {
                continue;
            }
            table = table.column(eguiColumn::auto());
        }

        table
            .header(20.0, |mut header| {
                for column_name in &app.res_columns {
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
                                    .button(if app.reversed && &app.sort_by_col == column_name {
                                        "⬆"
                                    } else if &app.sort_by_col == column_name {
                                        "⬇"
                                    } else {
                                        "⬆⬇"
                                    })
                                    .clicked()
                                {
                                    if &app.sort_by_col != column_name {
                                        app.sort_by_col = String::from(column_name);
                                        app.reversed = false;
                                    } else if !app.reversed {
                                        app.reversed = true;
                                    } else if app.reversed {
                                        app.reversed = false;
                                        app.sort_by_col = String::from(ROSEMARY_SORT_COL_STR);
                                    };

                                    if let Some(id_index) =
                                        app.res_columns.iter().position(|c| c == &app.sort_by_col)
                                    {
                                        // TODO: simplify this logic
                                        app.parsed_res_rows.sort_by(|a, b| {
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

                                            if app.reversed {
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

                let start_index = app.current_page * app.rows_per_page;
                let end_index = (start_index + app.rows_per_page).min(app.parsed_res_rows.len());
                let total_rows = if end_index > 0 {
                    end_index - start_index
                } else {
                    0
                };

                body.rows(text_height, total_rows, |mut row| {
                    if let Some(row_data) = app.parsed_res_rows.get(start_index + row.index()) {
                        for cell in row_data.iter().take(app.res_columns.len() - 1) {
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
