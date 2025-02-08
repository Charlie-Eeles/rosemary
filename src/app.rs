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

fn get_env_var_or_exit(name: &str) -> String {
    match std::env::var(name) {
        Ok(val) => val,
        Err(_) => {
            println!("Required variable not set in environment: {name}");
            std::process::exit(1);
        }
    }
}

fn format_sql(sql: &str) -> String {
    format(sql, &QueryParams::None, FormatOptions::default())
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
#[serde(default)]
pub struct Rosemary {
    code: String,
    #[serde(skip)]
    current_page: usize,
    #[serde(skip)]
    rows_per_page: usize,
    #[serde(skip)]
    db_pool: Option<Pool<Postgres>>,
    #[serde(skip)]
    res_columns: Vec<String>,
    #[serde(skip)]
    parsed_res_rows: Vec<Vec<CellValue>>,
    #[serde(skip)]
    reversed: bool,
}

impl Default for Rosemary {
    fn default() -> Self {
        Self {
            code: "".to_owned(),
            db_pool: None,
            res_columns: vec![String::from("")],
            parsed_res_rows: Vec::new(),
            current_page: 0,
            rows_per_page: 1000,
            reversed: true,
        }
    }
}

impl Rosemary {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        if let Some(storage) = cc.storage {
            let mut app: Rosemary = eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
            app.initialise_db();
            return app;
        }

        let mut app = Rosemary::default();
        app.initialise_db();
        app
    }

    fn initialise_db(&mut self) {
        let runtime = Runtime::new().expect("Failed to create runtime");
        let database_url = get_env_var_or_exit("DATABASE_URL");
        self.db_pool = Some(runtime.block_on(async {
            sqlx::PgPool::connect(&database_url)
                .await
                .expect("Failed to connect to db")
        }))
    }
}

impl eframe::App for Rosemary {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        catppuccin_egui::set_theme(ctx, catppuccin_egui::MOCHA);

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
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

            ui.add(
                egui::TextEdit::multiline(&mut self.code)
                    .font(egui::TextStyle::Monospace)
                    .code_editor()
                    .desired_rows(40)
                    .lock_focus(true)
                    .desired_width(f32::INFINITY)
                    .layouter(&mut layouter),
            );

            if ctx.input(|i| {
                i.key_pressed(egui::Key::Enter) && (i.modifiers.command || i.modifiers.ctrl)
            }) {
                should_execute = true;
            }

            if ui.add(egui::Button::new("Format")).clicked() {
                self.code = format_sql(&self.code);
            }

            if ui.add(egui::Button::new("Execute")).clicked() {
                should_execute = true;
            }
        });

        if should_execute && !self.code.trim().is_empty() {
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
                                let col_names: Vec<String> = rows[0]
                                    .columns()
                                    .iter()
                                    .map(|col| String::from(col.name()))
                                    .collect();
                                *col_res_ref = col_names;
                                let mut parsed_values: Vec<Vec<CellValue>> = Vec::new();
                                for row in rows {
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
                                    parsed_values.push(row_values);
                                }

                                *parsed_row_res_ref = parsed_values;
                                ctx_ref.request_repaint();
                            }
                        }
                        Err(e) => {
                            *col_res_ref = vec![String::from("error_message")];
                            *parsed_row_res_ref = vec![vec![CellValue::Text(format!("{e}"))]];
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

                for _ in &self.res_columns {
                    table = table.column(eguiColumn::auto());
                }

                table
                    .header(20.0, |mut header| {
                        for column_name in &self.res_columns {
                            header.col(|ui| {
                                egui::Sides::new().show(
                                    ui,
                                    |ui| {
                                        ui.strong(column_name);
                                    },
                                    |ui| {
                                        // TODO: make this work with more than just the ID column
                                        if column_name == "id" {
                                            if ui
                                                .button(if self.reversed { "⬆" } else { "⬇" })
                                                .clicked()
                                            {
                                                self.reversed = !self.reversed;

                                                if let Some(id_index) =
                                                    self.res_columns.iter().position(|c| c == "id")
                                                {
                                                    // TODO: simplify this logic
                                                    self.parsed_res_rows.sort_by(|a, b| {
                                                        let a_id = match &a[id_index] {
                                                            CellValue::SmallInt(v) => *v as i64,
                                                            CellValue::MedInt(v) => *v as i64,
                                                            CellValue::BigInt(v) => *v,
                                                            CellValue::Text(v) => {
                                                                v.parse::<i64>().unwrap_or(0)
                                                            }
                                                            _ => 0,
                                                        };

                                                        let b_id = match &b[id_index] {
                                                            CellValue::SmallInt(v) => *v as i64,
                                                            CellValue::MedInt(v) => *v as i64,
                                                            CellValue::BigInt(v) => *v,
                                                            CellValue::Text(v) => {
                                                                v.parse::<i64>().unwrap_or(0)
                                                            }
                                                            _ => 0,
                                                        };

                                                        if self.reversed {
                                                            b_id.cmp(&a_id)
                                                        } else {
                                                            a_id.cmp(&b_id)
                                                        }
                                                    });
                                                }
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
                                for cell in row_data {
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
    }
}
