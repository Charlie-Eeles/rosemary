use egui::Layout;
use egui_extras::{Column as eguiColumn, TableBuilder};
use sqlformat::QueryParams;
use sqlformat::{format, FormatOptions};
use sqlx::Column;
use sqlx::Row;
use sqlx::ValueRef;
use sqlx::{Pool, Postgres};
use tokio::runtime::Runtime;

use crate::postgres::convert_type;
use crate::postgres::CellValue;
use crate::queries::{get_public_tables, PublicTable};

const ROSEMARY_SORT_COL_STR: &str = "__rosemary_default_sort_by_col";

fn format_sql(sql: &str) -> String {
    format(
        sql,
        &QueryParams::None,
        FormatOptions {
            indent: sqlformat::Indent::Spaces(2),
            uppercase: true,
            lines_between_queries: 1,
        },
    )
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
#[serde(default)]
pub struct Rosemary {
    // Connection management
    // Persist on reload
    db_host: String,
    db_port: String,
    db_user: String,
    db_password: String,
    db_name: String,
    // Don't persist on reload
    #[serde(skip)]
    db_pool: Option<Pool<Postgres>>,
    #[serde(skip)]
    connection_modal_open: bool,

    // Code editor
    code: String,

    // Query result panel
    #[serde(skip)]
    current_page: usize,
    #[serde(skip)]
    rows_per_page: usize,
    #[serde(skip)]
    res_columns: Vec<String>,
    #[serde(skip)]
    parsed_res_rows: Vec<Vec<CellValue>>,
    #[serde(skip)]
    reversed: bool,
    #[serde(skip)]
    sort_by_col: String,

    // Table list
    #[serde(skip)]
    tables: Vec<PublicTable>,
    #[serde(skip)]
    should_fetch_table_list: bool,
    #[serde(skip)]
    table_filter: String,
}

impl Default for Rosemary {
    fn default() -> Self {
        Self {
            code: "".to_owned(),
            db_pool: None,
            res_columns: vec![String::new()],
            parsed_res_rows: Vec::new(),
            current_page: 0,
            rows_per_page: 1000,
            reversed: true,
            sort_by_col: String::from(ROSEMARY_SORT_COL_STR),
            tables: Vec::new(),
            should_fetch_table_list: false,
            table_filter: String::new(),
            connection_modal_open: false,
            db_host: "localhost".to_string(),
            db_port: "5432".to_string(),
            db_user: "".to_string(),
            db_password: "".to_string(),
            db_name: "".to_string(),
        }
    }
}

impl Rosemary {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        if let Some(storage) = cc.storage {
            let app: Rosemary = eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
            return app;
        }

        let app = Rosemary::default();
        app
    }

    fn reset_table_data(&mut self) {
        self.tables = Vec::new();
        self.should_fetch_table_list = true;
    }

    fn reset_query_result_data(&mut self) {
        self.res_columns = vec![String::new()];
        self.parsed_res_rows = Vec::new();
        self.current_page = 0;
        self.reversed = true;
        self.sort_by_col = String::from(ROSEMARY_SORT_COL_STR)
    }

    fn connect_to_db(&mut self) {
        let runtime = Runtime::new().expect("Failed to create runtime");
        let database_url = format!(
            "postgresql://{}:{}@{}:{}/{}",
            self.db_user, self.db_password, self.db_host, self.db_port, self.db_name
        );

        let connection_result =
            runtime.block_on(async { sqlx::PgPool::connect(&database_url).await });

        match connection_result {
            Ok(pool) => {
                self.db_pool = Some(pool);
                self.connection_modal_open = false;
                self.reset_table_data();
                self.reset_query_result_data();
            }
            Err(e) => {
                eprintln!("Failed to connect: {}", e);
            }
        }
    }

    fn get_tables(&mut self) {
        self.should_fetch_table_list = false;
        let db_pool = self.db_pool.clone();
        let table_rows_ref = &mut self.tables;

        let runtime = Runtime::new().expect("Failed to create runtime");
        runtime.block_on(async move {
            if let Some(pool) = db_pool {
                match get_public_tables(&pool).await {
                    Ok(rows) => {
                        if !rows.is_empty() {
                            *table_rows_ref = rows;
                        }
                    }
                    Err(e) => {
                        eprintln!("{:?}", e);
                    }
                }
            }
        });
    }
}

impl eframe::App for Rosemary {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if !self.db_pool.is_some() {
            self.connection_modal_open = true;
        }

        if self.db_pool.is_some() && self.should_fetch_table_list {
            self.get_tables();
        }

        catppuccin_egui::set_theme(ctx, catppuccin_egui::MOCHA);

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                    if ui.button("Connections").clicked() {
                        self.connection_modal_open = true;
                    }
                });
            });
        });

        let mut should_execute = false;

        egui::SidePanel::left("editor").show(ctx, |ui| {
            let theme =
                egui_extras::syntax_highlighting::CodeTheme::from_memory(ui.ctx(), ui.style());

            let mut layouter = |ui: &egui::Ui, string: &str, wrap_width: f32| {
                let mut layout_job = egui_extras::syntax_highlighting::highlight(
                    ui.ctx(),
                    ui.style(),
                    &theme,
                    string,
                    "sql".into(),
                );
                layout_job.wrap.max_width = wrap_width;
                ui.fonts(|f| f.layout_job(layout_job))
            };

            egui::ScrollArea::vertical()
                .id_salt("code_editor")
                .max_height(550.0)
                .show(ui, |ui| {
                    ui.add(
                        egui::TextEdit::multiline(&mut self.code)
                            .font(egui::TextStyle::Monospace)
                            .desired_rows(55)
                            .code_editor()
                            .desired_width(f32::INFINITY)
                            .layouter(&mut layouter),
                    );
                });

            if ctx.input(|i| {
                i.key_pressed(egui::Key::Enter) && (i.modifiers.command || i.modifiers.ctrl)
            }) {
                should_execute = true;
                println!("BLAH");
            }
            ui.with_layout(Layout::right_to_left(egui::Align::TOP), |ui| {
                if ui.add(egui::Button::new("Execute")).clicked() {
                    should_execute = true;
                }

                if ui.add(egui::Button::new("Format")).clicked() {
                    self.code = format_sql(&self.code);
                }
            });

            ui.separator();

            ui.horizontal(|ui| {
                ui.label("Filter:");
                ui.add(egui::TextEdit::singleline(&mut self.table_filter));
                self.table_filter = self.table_filter.to_lowercase();
                if ui.button("ｘ").clicked() {
                    self.table_filter.clear();
                }
            });

            egui::ScrollArea::vertical()
                .id_salt("public_table_list")
                .show(ui, |ui| {
                    for table in &self.tables {
                        let table_name = table.table_name.as_deref().unwrap_or("NULL");
                        if !self.table_filter.trim().is_empty()
                            && !table_name.to_lowercase().contains(&self.table_filter)
                        {
                            continue;
                        }
                        let table_type = table.table_type.as_deref().unwrap_or("NULL");
                        let button_label = format!("{table_name} [{table_type}]");
                        let button = egui::Button::new(button_label);

                        if ui.add_sized([ui.available_width(), 0.0], button).clicked() {
                            self.code = format_sql(&format!("SELECT * FROM {table_name};"));
                            should_execute = true;
                        }
                    }
                });
        });

        if should_execute && !self.code.trim().is_empty() {
            self.reset_query_result_data();
            let db_pool = self.db_pool.clone();
            let query_str = self.code.clone();
            let col_res_ref = &mut self.res_columns;
            let parsed_row_res_ref = &mut self.parsed_res_rows;
            let ctx_ref = ctx.clone();

            let runtime = Runtime::new().expect("Failed to create runtime");
            runtime.block_on(async move {
                if let Some(pool) = db_pool {
                    match sqlx::query(&query_str).fetch_all(&pool).await {
                        Ok(rows) => {
                            if !rows.is_empty() {
                                let mut col_names: Vec<String> = rows[0]
                                    .columns()
                                    .iter()
                                    .map(|col| String::from(col.name()))
                                    .collect();
                                col_names
                                    .insert(col_names.len(), String::from(ROSEMARY_SORT_COL_STR));
                                *col_res_ref = col_names;

                                let mut parsed_values: Vec<Vec<CellValue>> = Vec::new();
                                for (idx, row) in rows.iter().enumerate() {
                                    let mut row_values: Vec<CellValue> = Vec::new();
                                    for col in row.columns() {
                                        let col_type = col.type_info().to_string();
                                        let value = if row
                                            .try_get_raw(col.ordinal())
                                            .is_ok_and(|v| v.is_null())
                                        {
                                            CellValue::Null
                                        } else {
                                            convert_type(
                                                &col_type.to_uppercase().as_str(),
                                                &col,
                                                &row,
                                            )
                                        };

                                        row_values.push(value);
                                    }
                                    row_values.push(CellValue::BigInt(idx as i64));
                                    parsed_values.push(row_values);
                                }

                                *parsed_row_res_ref = parsed_values;
                                ctx_ref.request_repaint();
                            }
                        }
                        Err(e) => {
                            *col_res_ref = vec![
                                String::from("error_message"),
                                String::from(ROSEMARY_SORT_COL_STR),
                            ];
                            *parsed_row_res_ref = vec![vec![
                                CellValue::Text(format!("{e}")),
                                CellValue::BigInt(0 as i64),
                            ]];
                        }
                    }
                }
            });
        }

        egui::TopBottomPanel::bottom("pagination_panel").show(ctx, |ui| {
            if self.parsed_res_rows.len() > 1000 {
                ui.horizontal(|ui| {
                    if ui.button("Previous").clicked() {
                        if self.current_page > 0 {
                            self.current_page -= 1;
                        }
                    }

                    ui.label(format!(
                        "Page {}/{}",
                        self.current_page + 1,
                        (self.parsed_res_rows.len() + self.rows_per_page - 1) / self.rows_per_page
                    ));

                    if ui.button("Next").clicked() {
                        if (self.current_page + 1) * self.rows_per_page < self.parsed_res_rows.len()
                        {
                            self.current_page += 1;
                        }
                    }
                });
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::both().show(ui, |ui| {
                let mut table = TableBuilder::new(ui)
                    .striped(true)
                    .resizable(true)
                    .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                    .min_scrolled_height(0.0);

                for column_name in &self.res_columns {
                    if column_name == ROSEMARY_SORT_COL_STR {
                        continue;
                    }
                    table = table.column(eguiColumn::auto());
                }

                table
                    .header(20.0, |mut header| {
                        for column_name in &self.res_columns {
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
                                            .button(
                                                if self.reversed && &self.sort_by_col == column_name
                                                {
                                                    "⬆"
                                                } else if &self.sort_by_col == column_name {
                                                    "⬇"
                                                } else {
                                                    "⬆⬇"
                                                },
                                            )
                                            .clicked()
                                        {
                                            if &self.sort_by_col != column_name {
                                                self.sort_by_col = String::from(column_name);
                                                self.reversed = false;
                                            } else if !self.reversed {
                                                self.reversed = true;
                                            } else if self.reversed {
                                                self.reversed = false;
                                                self.sort_by_col =
                                                    String::from(ROSEMARY_SORT_COL_STR);
                                            };

                                            if let Some(id_index) = self
                                                .res_columns
                                                .iter()
                                                .position(|c| c == &self.sort_by_col)
                                            {
                                                // TODO: simplify this logic
                                                self.parsed_res_rows.sort_by(|a, b| {
                                                    let a_key = match &a[id_index] {
                                                        CellValue::SmallInt(v) => {
                                                            (0, Some(*v as i64), None::<&String>)
                                                        }
                                                        CellValue::MedInt(v) => {
                                                            (0, Some(*v as i64), None)
                                                        }
                                                        CellValue::BigInt(v) => (0, Some(*v), None),
                                                        CellValue::Text(v) => (1, None, Some(v)),
                                                        _ => (2, None, None),
                                                    };

                                                    let b_key = match &b[id_index] {
                                                        CellValue::SmallInt(v) => {
                                                            (0, Some(*v as i64), None)
                                                        }
                                                        CellValue::MedInt(v) => {
                                                            (0, Some(*v as i64), None)
                                                        }
                                                        CellValue::BigInt(v) => (0, Some(*v), None),
                                                        CellValue::Text(v) => (1, None, Some(v)),
                                                        _ => (2, None, None),
                                                    };

                                                    if self.reversed {
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

                        let start_index = self.current_page * self.rows_per_page;
                        let end_index =
                            (start_index + self.rows_per_page).min(self.parsed_res_rows.len());
                        let total_rows = if end_index > 0 {
                            end_index - start_index
                        } else {
                            0
                        };

                        body.rows(text_height, total_rows, |mut row| {
                            if let Some(row_data) =
                                self.parsed_res_rows.get(start_index + row.index())
                            {
                                for cell in row_data.iter().take(self.res_columns.len() - 1) {
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
        });
        let mut connect_to_db = false;

        if self.connection_modal_open {
            egui::Window::new("Connections")
                .collapsible(false)
                .resizable(false)
                .open(&mut self.connection_modal_open)
                .show(ctx, |ui| {
                    ui.label("Database Host:");
                    ui.text_edit_singleline(&mut self.db_host);
                    ui.label("Port:");
                    ui.text_edit_singleline(&mut self.db_port);
                    ui.label("Username:");
                    ui.text_edit_singleline(&mut self.db_user);
                    ui.label("Password:");
                    ui.text_edit_singleline(&mut self.db_password);
                    ui.label("Database Name:");
                    ui.text_edit_singleline(&mut self.db_name);

                    if ui.button("Connect").clicked() {
                        connect_to_db = true;
                    }
                });
        }
        if connect_to_db {
            self.connect_to_db();
        }
    }
}
