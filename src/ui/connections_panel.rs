use crate::Rosemary;
use egui::Ui;

pub fn show_connections_panel(ui: &mut Ui, app: &mut Rosemary) -> bool {
    let mut connect_to_db = false;

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

    if ui.button("Connect").clicked() {
        connect_to_db = true;
    }

    connect_to_db
}
