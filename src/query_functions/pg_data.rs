use std::net::Ipv4Addr;

use serde::{Deserialize, Serialize};
use sqlx::{postgres::types::Oid, PgConnection, Pool, Postgres};

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

#[derive(Serialize, Deserialize, Debug)]
pub struct RunningQueriesData {
    pub datid: Option<Oid>,
    pub datname: Option<String>,
    pub pid: Option<i32>,
    pub state: Option<String>,
    pub query: Option<String>,
    pub usesysid: Option<Oid>,
    pub usename: Option<String>,
    pub application_name: Option<String>,
    //pub client_addr: Option<Ipv4Addr>,
    pub client_port: Option<i32>,
    //pub query_start: Option<String>,
}

pub async fn get_running_queries_data(db: &Pool<Postgres>) -> Result<Vec<RunningQueriesData>, sqlx::Error> {
    sqlx::query_as!(
        RunningQueriesData,
        "
        SELECT
          datid,
          datname,
          pid,
          state,
          query,
          usesysid,
          usename,
          application_name,
          client_port
        FROM
          pg_stat_activity
        WHERE
          state = 'active';
        "
    )
    .fetch_all(db)
    .await
}
