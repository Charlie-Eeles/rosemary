use std::time::Instant;

use crate::postgres::convert_type;
use crate::postgres::CellValue;
use crate::queries::{get_public_tables, PublicTable};
use crate::ui::connections_panel::show_connections_panel;
use crate::ui::editor_panel::show_editor_panel;
use crate::ui::pagination_panel::show_pagination_panel;
use crate::ui::query_metrics_panel::show_query_metrics_panel;
use crate::ui::results_table_panel::show_results_table_panel;
use crate::ui::tables_panel::show_tables_panel;
use rayon::prelude::*;
use sqlformat::QueryParams;
use sqlformat::{format, FormatOptions};
use sqlx::postgres::PgRow;
use sqlx::Column;
use sqlx::Row;
use sqlx::{Pool, Postgres};
use tokio::runtime::Runtime;

pub const ROSEMARY_SORT_COL_STR: &str = "__rosemary_default_sort_by_col";

pub fn format_sql(sql: &str) -> String {
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

#[derive(Debug)]
pub struct QueryResultsPanel {
    pub current_page: usize,
    pub rows_per_page: usize,
    pub res_columns: Vec<String>,
    pub parsed_res_rows: Vec<Vec<CellValue>>,
    pub reversed: bool,
    pub sort_by_col: String,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
#[serde(default)]
pub struct Rosemary {
    // Connection management
    // Persist on reload
    pub db_host: String,
    pub db_port: String,
    pub db_user: String,
    pub db_password: String,
    pub db_name: String,
    // Don't persist on reload
    #[serde(skip)]
    pub db_pool: Option<Pool<Postgres>>,
    #[serde(skip)]
    pub connection_modal_open: bool,

    // Code editor
    pub code: String,
    pub query_to_execute: u8,

    #[serde(skip)]
    pub split_results_table: bool,

    #[serde(skip)]
    pub query_results: Vec<QueryResultsPanel>,

    // Table list
    #[serde(skip)]
    pub tables: Vec<PublicTable>,
    #[serde(skip)]
    pub should_fetch_table_list: bool,
    #[serde(skip)]
    pub table_filter: String,
    pub show_table_list: bool,
    pub table_queries_are_additive: bool,

    // Query performance panel
    #[serde(skip)]
    pub query_execution_time_ms: u128,
    #[serde(skip)]
    pub query_execution_time_sec: f64,
}

impl Default for Rosemary {
    fn default() -> Self {
        Self {
            code: "".to_owned(),
            query_to_execute: 0,
            db_pool: None,
            tables: Vec::new(),
            should_fetch_table_list: false,
            table_filter: String::new(),
            show_table_list: true,
            connection_modal_open: false,
            db_host: "localhost".to_string(),
            db_port: "5432".to_string(),
            db_user: "".to_string(),
            db_password: "".to_string(),
            db_name: "".to_string(),
            query_execution_time_ms: 0,
            query_execution_time_sec: 0.0,
            table_queries_are_additive: true,
            split_results_table: false,
            query_results: vec![QueryResultsPanel {
                res_columns: vec![String::new()],
                parsed_res_rows: Vec::new(),
                current_page: 0,
                rows_per_page: 1000,
                reversed: true,
                sort_by_col: String::from(ROSEMARY_SORT_COL_STR),
            }],
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

    fn reset_query_result_data(&mut self, idx: usize) {
        self.query_results[idx] = QueryResultsPanel {
            res_columns: vec![String::new()],
            parsed_res_rows: Vec::new(),
            current_page: 0,
            rows_per_page: 1000,
            reversed: true,
            sort_by_col: String::from(ROSEMARY_SORT_COL_STR),
        }
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
                self.reset_query_result_data(0);
                if self.query_results.len() > 1 {
                    self.reset_query_result_data(1);
                }
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
        let mut should_execute_secondary = false;

        for key in 0..=9 {
            let num_key = match key {
                0 => egui::Key::Num0,
                1 => egui::Key::Num1,
                2 => egui::Key::Num2,
                3 => egui::Key::Num3,
                4 => egui::Key::Num4,
                5 => egui::Key::Num5,
                6 => egui::Key::Num6,
                7 => egui::Key::Num7,
                8 => egui::Key::Num8,
                9 => egui::Key::Num9,
                _ => unreachable!(),
            };

            if ctx.input(|i| i.key_pressed(num_key) && (i.modifiers.command || i.modifiers.ctrl)) {
                self.query_to_execute = key;
            }
        }

        if ctx
            .input(|i| i.key_pressed(egui::Key::Enter) && (i.modifiers.command || i.modifiers.ctrl))
        {
            if ctx.input(|i| i.modifiers.shift) {
                should_execute_secondary = true;
            } else {
                should_execute = true;
            }
        }

        egui::SidePanel::left("editor").show(ctx, |ui| {
            show_editor_panel(ui, self, &mut should_execute);
            ui.separator();
            if self.show_table_list {
                show_tables_panel(ui, self, &mut should_execute);
            }
        });

        if (should_execute || should_execute_secondary) && !self.code.trim().is_empty() {
            let query_vec: Vec<&str> = self
                .code
                .split(';')
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .collect();

            let idx = self.query_to_execute.saturating_sub(1);
            let query_str = if self.query_to_execute == 0 {
                query_vec
                    .last()
                    .map_or_else(String::new, |s| String::from(*s))
            } else if let Some(index) = query_vec.get(idx as usize) {
                String::from(*index)
            } else {
                query_vec
                    .last()
                    .map_or_else(String::new, |s| String::from(*s))
            };

            //TODO: Make this work more dynamically, maybe the more things could be kept in vectors
            //to allow for tabs in the future...
            let query_idx = if should_execute { 0 } else { 1 };

            self.reset_query_result_data(query_idx);
            let db_pool = self.db_pool.clone();
            let query_execution_time_ms_ref = &mut self.query_execution_time_ms;
            let query_execution_time_sec_ref = &mut self.query_execution_time_sec;

            let runtime = Runtime::new().expect("Failed to create runtime");

            let (res_rows, error_message) = runtime.block_on(async {
                let mut res_rows: Vec<PgRow> = Vec::new();
                let mut error_message: String = String::new();

                if let Some(pool) = db_pool {
                    let query_start_time = Instant::now();
                    match sqlx::query(&query_str).fetch_all(&pool).await {
                        Ok(rows) => {
                            let elapsed_time = query_start_time.elapsed();
                            *query_execution_time_ms_ref = elapsed_time.as_millis();
                            *query_execution_time_sec_ref =
                                (elapsed_time.as_secs_f64() * 100.0).round() / 100.0;
                            res_rows = rows;
                        }
                        Err(e) => {
                            error_message = format!("{e}");
                        }
                    }
                }
                (res_rows, error_message)
            });

            if !error_message.is_empty() {
                self.query_results[query_idx].res_columns = vec![
                    String::from("error_message"),
                    String::from(ROSEMARY_SORT_COL_STR),
                ];
                self.query_results[query_idx].parsed_res_rows = vec![vec![
                    CellValue::Text(error_message),
                    CellValue::BigInt(0 as i64),
                ]];
            } else if !res_rows.is_empty() {
                let mut col_names: Vec<String> = res_rows[0]
                    .columns()
                    .iter()
                    .map(|col| String::from(col.name()))
                    .collect();
                col_names.insert(col_names.len(), String::from(ROSEMARY_SORT_COL_STR));
                self.query_results[query_idx].res_columns = col_names;

                self.query_results[query_idx].parsed_res_rows = res_rows
                    .par_iter()
                    .enumerate()
                    .map(|(idx, row)| {
                        let mut row_values: Vec<CellValue> = row
                            .columns()
                            .iter()
                            .map(|col| {
                                convert_type(
                                    col.type_info().to_string().to_uppercase().as_str(),
                                    col,
                                    &row,
                                )
                            })
                            .collect();
                        row_values.push(CellValue::BigInt(idx as i64));
                        row_values
                    })
                    .collect();
            }
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            let max_height = if self.split_results_table {
                ui.available_height() / 2.0
            } else {
                ui.available_height()
            };

            ui.push_id("top_table", |ui| {
                ui.set_min_height(max_height);
                ui.set_max_height(max_height);
                show_results_table_panel(ui, &mut self.query_results[0]);
            });

            if self.split_results_table {
                if self.query_results.len() >= 1 {
                    self.query_results.push(QueryResultsPanel {
                        res_columns: vec![String::new()],
                        parsed_res_rows: Vec::new(),
                        current_page: 0,
                        rows_per_page: 1000,
                        reversed: true,
                        sort_by_col: String::from(ROSEMARY_SORT_COL_STR),
                    })
                }

                ui.separator();

                ui.push_id("bottom_table", |ui| {
                    ui.set_min_height(max_height);
                    ui.set_max_height(max_height);
                    show_results_table_panel(ui, &mut self.query_results[1]);
                });
            }
        });

        egui::TopBottomPanel::bottom("pagination_panel").show(ctx, |ui| {
            show_query_metrics_panel(ui, self);
            show_pagination_panel(ui, &mut self.query_results[0]);
        });
        let mut connect_to_db = false;

        if self.connection_modal_open {
            let mut connections_modal_open = self.connection_modal_open;
            egui::Window::new("Connections")
                .collapsible(false)
                .resizable(false)
                .open(&mut connections_modal_open)
                .show(ctx, |ui| {
                    connect_to_db = show_connections_panel(ui, self);
                });
            self.connection_modal_open = connections_modal_open;
        }

        if connect_to_db {
            self.connect_to_db();
        }
    }
}
