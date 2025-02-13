use crate::app::Rosemary;
use egui::Ui;

pub fn show_pagination_panel(ui: &mut Ui, app: &mut Rosemary) {
    if app.parsed_res_rows.len() > 1000 {
        ui.horizontal(|ui| {
            if ui.button("Previous").clicked() {
                if app.current_page > 0 {
                    app.current_page -= 1;
                }
            }

            ui.label(format!(
                "Page {}/{}",
                app.current_page + 1,
                (app.parsed_res_rows.len() + app.rows_per_page - 1) / app.rows_per_page
            ));

            if ui.button("Next").clicked() {
                if (app.current_page + 1) * app.rows_per_page < app.parsed_res_rows.len() {
                    app.current_page += 1;
                }
            }
        });
    }
}
