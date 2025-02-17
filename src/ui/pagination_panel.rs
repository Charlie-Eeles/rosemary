use crate::Rosemary;
use egui::{Layout, Ui};

pub fn show_pagination_panel(ui: &mut Ui, app: &mut Rosemary) {
    if app.query_results[0].parsed_res_rows.len() > 1000
        || (app.split_results_table && app.query_results[1].parsed_res_rows.len() > 1000)
    {
        ui.horizontal(|ui| {
            if ui.button("Previous").clicked() {
                if app.query_results[0].current_page > 0 {
                    app.query_results[0].current_page -= 1;
                }
            }

            ui.label(format!(
                "Page {}/{}",
                app.query_results[0].current_page + 1,
                (app.query_results[0].parsed_res_rows.len() + app.query_results[0].rows_per_page
                    - 1)
                    / app.query_results[0].rows_per_page
            ));

            if ui.button("Next").clicked() {
                if (app.query_results[0].current_page + 1) * app.query_results[0].rows_per_page
                    < app.query_results[0].parsed_res_rows.len()
                {
                    app.query_results[0].current_page += 1;
                }
            }

            ui.with_layout(Layout::right_to_left(egui::Align::TOP), |ui| {
                if app.split_results_table && app.query_results[1].parsed_res_rows.len() > 1000 {
                    if ui.button("Next").clicked() {
                        if (app.query_results[1].current_page + 1)
                            * app.query_results[1].rows_per_page
                            < app.query_results[1].parsed_res_rows.len()
                        {
                            app.query_results[1].current_page += 1;
                        }
                    }

                    ui.label(format!(
                        "Page {}/{}",
                        app.query_results[1].current_page + 1,
                        (app.query_results[1].parsed_res_rows.len()
                            + app.query_results[1].rows_per_page
                            - 1)
                            / app.query_results[1].rows_per_page
                    ));

                    if ui.button("Previous").clicked() {
                        if app.query_results[1].current_page > 0 {
                            app.query_results[1].current_page -= 1;
                        }
                    }
                }
            })
        });
    }
}
