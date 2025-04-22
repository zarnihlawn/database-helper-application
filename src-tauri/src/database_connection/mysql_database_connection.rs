use serde::{Deserialize, Serialize};
use sqlx::{mysql::MySqlPool, Row, SqlitePool};
use std::collections::HashMap;

use super::app_database_connection::get_db_path;

#[derive(Debug, Serialize, Deserialize)]
pub struct ColumnInfo {
    name: String,
    data_type: String,
    is_nullable: String,
    column_default: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TableInfo {
    name: String,
    columns: Vec<ColumnInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DatabaseObjects {
    tables: Vec<TableInfo>,
}

#[tauri::command]
pub async fn test_mysql_connection(url: String) -> Result<String, String> {
    let pool = MySqlPool::connect(&url).await.map_err(|e| e.to_string())?;

    let _ = sqlx::query("SELECT 1")
        .fetch_all(&pool)
        .await
        .map_err(|e| e.to_string())?;

    Ok("Connected".to_string())
}

#[tauri::command]
pub async fn save_mysql_connection(
    user_id: Option<i64>,
    url: String,
    connection_name: String,
) -> Result<(), String> {
    // Only insert if the connection test succeeds
    test_mysql_connection(url.clone()).await.map_err(|e| e)?;

    let app_database = get_db_path();
    let pool = SqlitePool::connect(&format!("sqlite://{}", app_database))
        .await
        .unwrap();

    if let Some(uid) = user_id {
        // Insert with user_id
        sqlx::query("INSERT INTO database_connection (user_id, datasource_id, connection_name, url) VALUES (?, ?, ?, ?)")
            .bind(uid)
            .bind(4)
            .bind(connection_name.as_str())
            .bind(url.as_str())
            .execute(&pool)
            .await
            .map_err(|e| e.to_string())?;
    } else {
        // Insert without user_id
        sqlx::query("INSERT INTO database_connection (datasource_id, connection_name, url) VALUES (?, ?, ?)")
            .bind(4)
            .bind(connection_name.as_str())
            .bind(url.as_str())
            .execute(&pool)
            .await
            .map_err(|e| e.to_string())?;
    }

    Ok(())
}

#[tauri::command]
pub async fn get_database_from_mysql(
    url: String,
) -> Result<HashMap<String, HashMap<String, DatabaseObjects>>, String> {
    let pool = MySqlPool::connect(&url).await.map_err(|e| e.to_string())?;

    // Get all schemas
    let schemas = sqlx::query("SELECT SCHEMA_NAME FROM information_schema.SCHEMATA")
        .fetch_all(&pool)
        .await
        .map_err(|e| e.to_string())?;

    let mut database_schemas = HashMap::new();
    for schema in schemas {
        let schema_name: String = schema.get::<String, _>("SCHEMA_NAME");

        // Get all tables for this schema
        let tables = sqlx::query(
            "
            SELECT TABLE_NAME, TABLE_TYPE
            FROM information_schema.TABLES 
            WHERE TABLE_SCHEMA = ?
        ",
        )
        .bind(&schema_name)
        .fetch_all(&pool)
        .await
        .unwrap_or_default();

        let mut table_infos = Vec::new();
        for table in tables {
            let table_name: String = table.get::<String, _>("TABLE_NAME");

            // Get columns for this table
            let columns = sqlx::query(
                "
                SELECT 
                    COLUMN_NAME,
                    DATA_TYPE,
                    IS_NULLABLE,
                    COLUMN_DEFAULT
                FROM information_schema.COLUMNS 
                WHERE TABLE_SCHEMA = ? AND TABLE_NAME = ?
                ORDER BY ORDINAL_POSITION
            ",
            )
            .bind(&schema_name)
            .bind(&table_name)
            .fetch_all(&pool)
            .await
            .unwrap_or_default();

            let column_infos: Vec<ColumnInfo> = columns
                .iter()
                .map(|row| ColumnInfo {
                    name: row.get::<String, _>("COLUMN_NAME"),
                    data_type: row.get::<String, _>("DATA_TYPE"),
                    is_nullable: row.get::<String, _>("IS_NULLABLE"),
                    column_default: row.get::<Option<String>, _>("COLUMN_DEFAULT"),
                })
                .collect();

            table_infos.push(TableInfo {
                name: table_name,
                columns: column_infos,
            });
        }

        let mut schema_objects = HashMap::new();
        schema_objects.insert(
            schema_name.clone(),
            DatabaseObjects {
                tables: table_infos,
            },
        );

        database_schemas.insert(schema_name, schema_objects);
    }

    Ok(database_schemas)
}
