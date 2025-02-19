use crate::Rosemary;
use egui::Ui;

pub fn show_databases_panel(ui: &mut Ui, app: &mut Rosemary) -> bool {
    let mut connect_to_db = false;

    egui::ScrollArea::vertical()
        .id_salt("connection_list")
        .show(ui, |ui| {
            for db in &app.databases {
                let db_name = db.datname.as_deref().unwrap_or("");
                let button = egui::Button::new(db_name);
                if ui.add_sized([ui.available_width(), 0.0], button).clicked() {
                    app.selected_db = String::from(db_name);
                    connect_to_db = true;
                }
            }
        });

    connect_to_db
}
