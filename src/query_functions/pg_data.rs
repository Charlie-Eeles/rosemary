use sqlx::{Pool, Postgres};
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug)]
pub struct PublicTable {
    pub table_name: Option<String>,
    pub table_type: Option<String>,
    pub table_schema: Option<String>,
}

pub async fn get_public_tables(db: &Pool<Postgres>) -> Result<Vec<PublicTable>, sqlx::Error> {
    //TODO: make the schema name dynamic when schema selection is added
    sqlx::query_as!(
        PublicTable,
        "
        SELECT
          table_name,
          table_type,
          table_schema
        FROM
          information_schema.tables
        ORDER BY
          table_schema,
          table_type,
          table_name;
        "
    )
    .fetch_all(db)
    .await
}


#[derive(Serialize, Deserialize, Debug)]
pub struct DatabaseNames {
    pub datname: Option<String>,
}

pub async fn get_database_names(db: &Pool<Postgres>) -> Result<Vec<DatabaseNames>, sqlx::Error> {
    sqlx::query_as!(
        DatabaseNames,
        "
        SELECT
          datname
        FROM
          pg_database;
        "
    )
    .fetch_all(db)
    .await
}
