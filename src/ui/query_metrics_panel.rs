use crate::app::Rosemary;
use egui::Ui;

pub fn show_query_metrics_panel(ui: &mut Ui, app: &mut Rosemary) {
    ui.horizontal(|ui| {
        if app.query_results[0].query_execution_time_sec > 1.0 {
            ui.label(format!(
                "Execution time: {} sec",
                app.query_results[0].query_execution_time_sec
            ));
        } else {
            ui.label(format!(
                "Execution time: {} ms",
                app.query_results[0].query_execution_time_ms
            ));
        }

        if app.split_results_table {
            ui.label("//");

            if app.query_results[1].query_execution_time_sec > 1.0 {
                ui.label(format!(
                    "Execution time: {} sec",
                    app.query_results[1].query_execution_time_sec
                ));
            } else {
                ui.label(format!(
                    "Execution time: {} ms",
                    app.query_results[1].query_execution_time_ms
                ));
            }
        }
    });
}
