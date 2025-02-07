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
    match col_type {
        // -------------------- Strings --------------------
        "TEXT" | "VARCHAR" | "NAME" | "CITEXT" | "BPCHAR" | "CHAR" => row
            .try_get::<String, usize>(col.ordinal())
            .map(CellValue::Text)
            .unwrap_or(CellValue::Unsupported),

        // -------------------- Numbers --------------------
        "SMALLINT" | "SMALLSERIAL" | "INT2" => row
            .try_get::<i16, usize>(col.ordinal())
            .map(CellValue::SmallInt)
            .unwrap_or(CellValue::Unsupported),

        "INT" | "SERIAL" | "INT4" => row
            .try_get::<i32, usize>(col.ordinal())
            .map(CellValue::MedInt)
            .unwrap_or(CellValue::Unsupported),

        "BIGINT" | "BIGSERIAL" | "INT8" => row
            .try_get::<i64, usize>(col.ordinal())
            .map(CellValue::BigInt)
            .unwrap_or(CellValue::Unsupported),

        "REAL" | "FLOAT4" => row
            .try_get::<f32, usize>(col.ordinal())
            .map(CellValue::SmallFloat)
            .unwrap_or(CellValue::Unsupported),

        "DOUBLE PRECISION" | "FLOAT8" => row
            .try_get::<f64, usize>(col.ordinal())
            .map(CellValue::BigFloat)
            .unwrap_or(CellValue::Unsupported),

        "NUMERIC" => row
            .try_get::<BigDecimal, usize>(col.ordinal())
            .map(|num| CellValue::Text(num.to_string()))
            .unwrap_or(CellValue::Unsupported),

        // -------------------- Dates & Times --------------------
        "TIMESTAMPTZ" => row
            .try_get::<chrono::DateTime<chrono::Utc>, usize>(col.ordinal())
            .map(|dt| CellValue::Text(dt.to_rfc3339()))
            .unwrap_or(CellValue::Unsupported),

        "TIMESTAMP" => row
            .try_get::<chrono::DateTime<chrono::Utc>, usize>(col.ordinal())
            .map(|dt| CellValue::Text(dt.to_rfc3339()))
            .unwrap_or(CellValue::Unsupported),

        "DATE" => row
            .try_get::<chrono::DateTime<chrono::Utc>, usize>(col.ordinal())
            .map(|dt| CellValue::Text(dt.to_rfc3339()))
            .unwrap_or(CellValue::Unsupported),

        "TIME" => row
            .try_get::<chrono::DateTime<chrono::Utc>, usize>(col.ordinal())
            .map(|dt| CellValue::Text(dt.to_rfc3339()))
            .unwrap_or(CellValue::Unsupported),

        // -------------------- UUID --------------------
        "UUID" => row
            .try_get::<Uuid, usize>(col.ordinal())
            .map(CellValue::Uuid)
            .unwrap_or(CellValue::Unsupported),


        // -------------------- Other & Unknown --------------------
        _ => row
            .try_get_unchecked::<String, usize>(col.ordinal())
            .map(CellValue::Text)
            .unwrap_or(CellValue::Unsupported),
    }
}
