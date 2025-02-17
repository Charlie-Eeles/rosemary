use crate::app::Rosemary;
use egui::{Layout, Ui};
use num_format::{Locale, ToFormattedString};

pub fn show_query_metrics_panel(ui: &mut Ui, app: &mut Rosemary) {
    ui.horizontal(|ui| {
        let row_or_rows = if app.query_results[0].parsed_res_rows.len() == 1 {
            "Row"
        } else {
            "Rows"
        };
        let formatted_num_of_rows = app.query_results[0]
            .parsed_res_rows
            .len()
            .to_formatted_string(&Locale::en);
        if app.query_results[0].query_execution_time_sec > 1.0 {
            ui.label(format!(
                "{} {} || Execution time: {} sec",
                formatted_num_of_rows, row_or_rows, app.query_results[0].query_execution_time_sec
            ));
        } else {
            ui.label(format!(
                "{} {} || Execution time: {} ms",
                formatted_num_of_rows, row_or_rows, app.query_results[0].query_execution_time_ms
            ));
        }

        if app.split_results_table {
            ui.with_layout(Layout::right_to_left(egui::Align::TOP), |ui| {
                let row_or_rows = if app.query_results[1].parsed_res_rows.len() == 1 {
                    "Row"
                } else {
                    "Rows"
                };
                let formatted_num_of_rows = app.query_results[1]
                    .parsed_res_rows
                    .len()
                    .to_formatted_string(&Locale::en);
                if app.query_results[1].query_execution_time_sec > 1.0 {
                    ui.label(format!(
                        "{} {} || Execution time: {} sec",
                        formatted_num_of_rows,
                        row_or_rows,
                        app.query_results[1].query_execution_time_sec
                    ));
                } else {
                    ui.label(format!(
                        "{} {} || Execution time: {} ms",
                        formatted_num_of_rows,
                        row_or_rows,
                        app.query_results[1].query_execution_time_ms
                    ));
                }
            });
        }
    });
}
