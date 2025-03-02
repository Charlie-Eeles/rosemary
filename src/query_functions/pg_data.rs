use serde::{Deserialize, Serialize};
use sqlx::{PgConnection, Pool, Postgres};

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

#[derive(Serialize, Deserialize, Debug)]
pub struct QueryPID {
    pub pg_backend_pid: Option<i32>,
}

pub async fn get_query_pid(db: &mut PgConnection) -> Result<QueryPID, sqlx::Error> {
    sqlx::query_as!(QueryPID, "SELECT pg_backend_pid();")
        .fetch_one(db)
        .await
}


pub async fn cancel_query(db: &Pool<Postgres>, pid: i32) -> Result<(), sqlx::Error> {
    if pid > 0 {
        let query = format!("SELECT pg_terminate_backend({})", pid);
        sqlx::query(&query).execute(db).await?;
    }
    Ok(())
}
