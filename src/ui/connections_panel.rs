use crate::{app::SavedConnection, Rosemary};
use egui::Ui;

pub fn show_connections_panel(ui: &mut Ui, app: &mut Rosemary) -> bool {
    let mut connect_to_db = false;

    ui.label("Connection name:");
    ui.text_edit_singleline(&mut app.connection_name);
    ui.label("Database Host:");
    ui.text_edit_singleline(&mut app.db_host);
    ui.label("Port:");
    ui.text_edit_singleline(&mut app.db_port);
    ui.label("Username:");
    ui.text_edit_singleline(&mut app.db_user);
    ui.label("Password:");
    ui.horizontal(|ui| ui.add(egui::TextEdit::singleline(&mut app.db_password).password(true)));
    ui.label("Database Name:");
    ui.text_edit_singleline(&mut app.db_name);

    if ui.button("Save").clicked() {
        let connection = SavedConnection {
            connection_name: app.connection_name.clone(),
            db_host: app.db_host.clone(),
            db_port: app.db_port.clone(),
            db_user: app.db_user.clone(),
            db_password: app.db_password.clone(),
            db_name: app.db_name.clone(),
        };
        app.connection_list.push(connection);
    }

    let mut idx_to_remove: usize = 0;
    let mut delete_connection = false;

    egui::ScrollArea::vertical()
        .id_salt("connection_list")
        .show(ui, |ui| {
            for (idx, connection) in app.connection_list.iter().enumerate() {
                ui.horizontal(|ui| {
                    if ui.button(&connection.connection_name).clicked() {
                        app.connect_to_idx = idx;
                        connect_to_db = true;
                    }
                    if ui.button("ｘ").clicked() {
                        idx_to_remove = idx;
                        delete_connection = true;
                    }
                });
            }
        });

    if delete_connection {
        app.connection_list.remove(idx_to_remove);
    }

    connect_to_db
}
