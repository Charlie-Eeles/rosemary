use sqlx::postgres::{PgColumn, PgRow};
use sqlx::types::{BigDecimal, Uuid};
use sqlx::Column;
use sqlx::Row;

#[derive(Debug)]
pub enum CellValue {
    Text(String),
    SmallInt(i16),
    MedInt(i32),
    BigInt(i64),
    SmallFloat(f32),
    BigFloat(f64),
    Uuid(Uuid),
    BigDecimal(BigDecimal),
    Null,
    Unsupported,
}

pub fn convert_type(col_type: &str, col: &PgColumn, row: &PgRow) -> CellValue {
    let ord = col.ordinal();

    if let Ok(None) = row.try_get::<Option<()>, _>(ord) {
        return CellValue::Null;
    }

    match col_type {
        // -------------------- Strings --------------------
        "TEXT" | "VARCHAR" | "NAME" | "CITEXT" | "BPCHAR" | "CHAR" => row
            .try_get::<String, _>(ord)
            .map(CellValue::Text)
            .unwrap_or(CellValue::Unsupported),

        // -------------------- Numbers --------------------
        "SMALLINT" | "SMALLSERIAL" | "INT2" => row
            .try_get::<i16, _>(ord)
            .map(CellValue::SmallInt)
            .unwrap_or(CellValue::Unsupported),

        "INT" | "SERIAL" | "INT4" => row
            .try_get::<i32, _>(ord)
            .map(CellValue::MedInt)
            .unwrap_or(CellValue::Unsupported),

        "BIGINT" | "BIGSERIAL" | "INT8" => row
            .try_get::<i64, _>(ord)
            .map(CellValue::BigInt)
            .unwrap_or(CellValue::Unsupported),

        "REAL" | "FLOAT4" => row
            .try_get::<f32, _>(ord)
            .map(CellValue::SmallFloat)
            .unwrap_or(CellValue::Unsupported),

        "DOUBLE PRECISION" | "FLOAT8" => row
            .try_get::<f64, _>(ord)
            .map(CellValue::BigFloat)
            .unwrap_or(CellValue::Unsupported),

        "NUMERIC" => row
            .try_get::<BigDecimal, _>(ord)
            .map(|num| CellValue::Text(num.to_string()))
            .unwrap_or(CellValue::Unsupported),

        // -------------------- Dates & Times --------------------
        "TIMESTAMPTZ" => row
            .try_get::<chrono::DateTime<chrono::Utc>, _>(ord)
            .map(|dt| CellValue::Text(dt.to_rfc3339()))
            .unwrap_or(CellValue::Unsupported),

        "TIMESTAMP" => row
            .try_get::<chrono::DateTime<chrono::Utc>, _>(ord)
            .map(|dt| CellValue::Text(dt.to_rfc3339()))
            .unwrap_or(CellValue::Unsupported),

        "DATE" => row
            .try_get::<chrono::DateTime<chrono::Utc>, _>(ord)
            .map(|dt| CellValue::Text(dt.to_rfc3339()))
            .unwrap_or(CellValue::Unsupported),

        "TIME" => row
            .try_get::<chrono::DateTime<chrono::Utc>, _>(ord)
            .map(|dt| CellValue::Text(dt.to_rfc3339()))
            .unwrap_or(CellValue::Unsupported),

        // -------------------- UUID --------------------
        "UUID" => row
            .try_get::<Uuid, _>(ord)
            .map(CellValue::Uuid)
            .unwrap_or(CellValue::Unsupported),


        // -------------------- Other & Unknown --------------------
        _ => row
            .try_get_unchecked::<String, _>(ord)
            .map(CellValue::Text)
            .unwrap_or(CellValue::Unsupported),
    }
}
