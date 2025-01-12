use egui_extras::{Column as eguiColumn, TableBuilder};
use sqlx::Column;
use sqlx::Row;
use sqlx::{Pool, Postgres};
use tokio::runtime::Runtime;

fn get_env_var_or_exit(name: &str) -> String {
    match std::env::var(name) {
        Ok(val) => val,
        Err(_) => {
            println!("Required variable not set in environment: {name}");
            std::process::exit(1);
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct Rosemary {
    code: String,
    #[serde(skip)]
    db_pool: Option<Pool<Postgres>>,
    res_columns: Vec<String>,
}

impl Default for Rosemary {
    fn default() -> Self {
        Self {
            code: "".to_owned(),
            db_pool: None,
            res_columns: vec![String::from("")],
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
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
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
                    .desired_rows(10)
                    .lock_focus(true)
                    .desired_width(f32::INFINITY)
                    .layouter(&mut layouter),
            );

            if ui.add(egui::Button::new("Execute")).clicked() {
                if !self.code.trim().is_empty() {
                    let db_pool = self.db_pool.clone();
                    let query_str = self.code.clone();
                    let col_res_ref = &mut self.res_columns;
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
                                        ctx_ref.request_repaint();
                                    }
                                }
                                Err(e) => {
                                    println!("Query failed: {e}");
                                }
                            }
                        }
                    });
                }
            }

            ui.separator();

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
                            ui.strong(column_name);
                        });
                    }
                })
                .body(|body| {
                    let row_count = 100;
                    let text_height = 20.0;
                    body.rows(text_height, row_count, |mut row| {
                        for _ in &self.res_columns {
                            row.col(|ui| {
                                ui.label("...");
                            });
                        }
                    });
                });
        });
    }
}
