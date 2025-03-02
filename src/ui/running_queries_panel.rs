use crate::Rosemary;
use egui::Ui;
use egui_extras::{Column, TableBuilder};

pub fn show_running_queries_panel(ui: &mut Ui, app: &mut Rosemary) {
    TableBuilder::new(ui)
        .striped(true)
        .resizable(false)
        .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
        .min_scrolled_height(0.0)
        .column(Column::auto().resizable(true))
        .column(Column::auto().resizable(true))
        .column(Column::auto().resizable(true))
        .column(Column::auto().resizable(true))
        .column(
            Column::remainder()
                .at_least(40.0)
                .clip(true)
                .resizable(true),
        )
        .column(Column::auto().resizable(true))
        .column(Column::auto().resizable(true))
        .column(Column::auto().resizable(true))
        .column(Column::auto().resizable(true))
        .column(Column::auto().resizable(false))
        .header(20.0, |mut header| {
            header.col(|ui| {
                ui.heading("datid");
            });
            header.col(|ui| {
                ui.heading("datname");
            });
            header.col(|ui| {
                ui.heading("pid");
            });
            header.col(|ui| {
                ui.heading("state");
            });
            header.col(|ui| {
                ui.heading("query");
            });
            header.col(|ui| {
                ui.heading("usesysid");
            });
            header.col(|ui| {
                ui.heading("usename");
            });
            header.col(|ui| {
                ui.heading("application_name");
            });
            header.col(|ui| {
                ui.heading("client_port");
            });
        })
        .body(|mut body| {
            for query in &app.running_queries {
                body.row(20.0, |mut row| {
                    row.col(|ui| {
                        ui.label(format!("{:?}", query.datid.unwrap_or_default()));
                    });
                    row.col(|ui| {
                        ui.label(query.datname.as_deref().unwrap_or("N/A"));
                    });
                    row.col(|ui| {
                        ui.label(format!("{:?}", query.pid.unwrap_or_default()));
                    });
                    row.col(|ui| {
                        ui.label(query.state.as_deref().unwrap_or("N/A"));
                    });
                    row.col(|ui| {
                        ui.label(query.query.as_deref().unwrap_or("N/A"));
                    });
                    row.col(|ui| {
                        ui.label(format!("{:?}", query.usesysid.unwrap_or_default()));
                    });
                    row.col(|ui| {
                        ui.label(query.usename.as_deref().unwrap_or("N/A"));
                    });
                    row.col(|ui| {
                        ui.label(query.application_name.as_deref().unwrap_or("N/A"));
                    });
                    row.col(|ui| {
                        ui.label(format!("{:?}", query.client_port.unwrap_or_default()));
                    });
                });
            }
        });
}
