use crate::app::Rosemary;
use egui::Ui;

pub fn show_query_metrics_panel(ui: &mut Ui, app: &mut Rosemary) {
    ui.horizontal(|ui| {
        if app.query_execution_time_sec > 1.0 {
            ui.label(format!(
                "Execution time: {} sec",
                app.query_execution_time_sec
            ));
        } else {
            ui.label(format!(
                "Execution time: {} ms",
                app.query_execution_time_ms
            ));
        }
    });
}
