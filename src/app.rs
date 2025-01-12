use egui::TextWrapMode;
use egui_extras::{Column, TableBuilder};
use sqlx::{Pool, Postgres};
use tokio::runtime::Runtime;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct Rosemary {
    code: String,
    #[serde(skip)]
    db_pool: Option<Pool<Postgres>>,
    query_result: String,
}

impl Default for Rosemary {
    fn default() -> Self {
        Self {
            code: "".to_owned(),
            db_pool: None,
            query_result: "No results yet.".to_owned(),
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
        self.db_pool = Some(runtime.block_on(async {
            sqlx::PgPool::connect("")
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
                ui.add_space(16.0);

                egui::widgets::global_theme_preference_buttons(ui);
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
                if let Some(query_str) = self.code.clone().into() {
                    let db_pool = self.db_pool.clone();
                    let result_ref = &mut self.query_result;
                    let runtime = Runtime::new().expect("Failed to create runtime");

                    runtime.block_on(async move {
                        if let Some(pool) = db_pool {
                            match sqlx::query(&query_str).fetch_all(&pool).await {
                                Ok(rows) => {
                                    *result_ref = String::from(format!("Results: {:?}", rows));
                                    println!("{:?}", &result_ref);
                                }
                                Err(_) => {
                                    println!("Failed");
                                }
                            }
                        }
                    });
                }
            }

            ui.separator();

            let columns = [
                String::from("example"),
                String::from("example2"),
                String::from("example3"),
            ];

            let available_height = ui.available_height();
            let mut table = TableBuilder::new(ui)
                .striped(true)
                .resizable(true)
                .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                .min_scrolled_height(0.0)
                .max_scroll_height(available_height);

            for _ in &columns {
                table = table.column(Column::auto());
            }

            let text_height = 40 as f32;

            table
                .header(20.0, |mut header| {
                    for column_name in &columns {
                        header.col(|ui| {
                            ui.strong(column_name);
                        });
                    }
                })
                .body(|body| {
                    body.rows(text_height, 100, |mut row| {
                        let row_index = row.index() + 1;

                        row.set_selected(false);

                        row.col(|ui| {
                            ui.label(row_index.to_string());
                        });
                        row.col(|ui| {
                            ui.add(
                                egui::Label::new("Thousands of rows of even height")
                                    .wrap_mode(TextWrapMode::Extend),
                            );
                        });
                        //TODO: do the same as the table example for the rows here over the pgvec
                        //result
                    })
                });
        });
    }
}
