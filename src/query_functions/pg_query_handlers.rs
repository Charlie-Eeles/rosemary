use std::time::Instant;

use sqlx::{postgres::PgRow, Pool, Postgres};
use tokio::runtime::Runtime;

pub fn execute_query(db_pool: Option<Pool<Postgres>>, query_str: String) -> (Vec<PgRow>, String, u128, f64) {
    let runtime = Runtime::new().expect("Failed to create runtime");
    let mut query_execution_time_ms: u128 = 0;
    let mut query_execution_time_sec = 0.0;

    runtime.block_on(async {
        let mut res_rows: Vec<PgRow> = Vec::new();
        let mut error_message: String = String::new();

        if let Some(pool) = db_pool {
            let query_start_time = Instant::now();
            match sqlx::query(&query_str).fetch_all(&pool).await {
                Ok(rows) => {
                    let elapsed_time = query_start_time.elapsed();
                    query_execution_time_ms = elapsed_time.as_millis();
                    query_execution_time_sec = (elapsed_time.as_secs_f64() * 100.0).round() / 100.0;
                    res_rows = rows;
                }
                Err(e) => {
                    error_message = format!("{e}");
                }
            }
        }
        (
            res_rows,
            error_message,
            query_execution_time_ms,
            query_execution_time_sec,
        )
    })
}


