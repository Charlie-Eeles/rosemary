use sqlx::{Pool, Postgres};
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug)]
pub struct PublicTable {
    pub table_name: Option<String>,
    pub table_type: Option<String>,
}

pub async fn get_public_tables(db: &Pool<Postgres>) -> Result<Vec<PublicTable>, sqlx::Error> {
    sqlx::query_as!(
        PublicTable,
        "
        SELECT
          table_name, table_type
        FROM
          information_schema.tables
        WHERE
          table_schema = 'public'
        ORDER BY
          table_type, table_name;
        "
    )
    .fetch_all(db)
    .await
}
