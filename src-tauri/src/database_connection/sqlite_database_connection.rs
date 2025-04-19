// use crate::database_connection::app_database_connection::get_db_path;
// use crate::models::structs::erd_struct::{ColumnInfo, TableInfo};

use sqlx::sqlite::SqlitePool;
use sqlx::Row;

use crate::models::structs::erd_struct::{ColumnInfo, TableInfo};

use super::app_database_connection::get_db_path;

#[tauri::command]
pub async fn test_sqlite_connection(url: String) -> Result<String, String> {
    let pool = SqlitePool::connect(&format!("sqlite://{}", url))
        .await
        .map_err(|e| e.to_string())?;

    let _ = sqlx::query("SELECT 1")
        .fetch_all(&pool)
        .await
        .map_err(|e| e.to_string())?;

    Ok("Connected".to_string())
}

#[tauri::command]
pub async fn save_sqlite_connection(
    user_id: Option<i64>,
    url: String,
    connection_name: String,
) -> Result<(), String> {
    // Only insert if the connection test succeeds
    test_sqlite_connection(url.clone()).await.map_err(|e| e)?;

    let app_database = get_db_path();
    let pool = SqlitePool::connect(&format!("sqlite://{}", app_database))
        .await
        .unwrap();

    if let Some(uid) = user_id {
        // Insert with user_id
        sqlx::query("INSERT INTO database_connection (user_id, datasource_id, connection_name, url) VALUES (?, ?, ?, ?)")
            .bind(uid)
            .bind(2)
            .bind(connection_name.as_str())
            .bind(url.as_str())
            .execute(&pool)
            .await
            .map_err(|e| e.to_string())?;
    } else {
        // Insert without user_id
        sqlx::query("INSERT INTO database_connection (datasource_id, connection_name, url) VALUES (?, ?, ?)")
            .bind(2)
            .bind(connection_name.as_str())
            .bind(url.as_str())
            .execute(&pool)
            .await
            .map_err(|e| e.to_string())?;
    }

    Ok(())
}

#[tauri::command]
pub async fn get_database_from_sqlite(url: String) -> Result<Vec<TableInfo>, String> {
    let pool = SqlitePool::connect(&format!("sqlite://{}", url))
        .await
        .map_err(|e| e.to_string())?;

    // Get all tables
    let tables = sqlx::query("SELECT name FROM sqlite_master WHERE type='table'")
        .fetch_all(&pool)
        .await
        .map_err(|e| e.to_string())?;

    let mut table_infos = Vec::new();

    for table in tables {
        let table_name: String = table.get(0);

        // Get columns for each table
        let columns = sqlx::query(&format!("PRAGMA table_info('{}')", table_name))
            .fetch_all(&pool)
            .await
            .map_err(|e| e.to_string())?;

        let mut column_infos = Vec::new();
        for column in columns {
            column_infos.push(ColumnInfo {
                name: column.get(1),
                data_type: column.get(2),
            });
        }

        table_infos.push(TableInfo {
            name: table_name,
            columns: column_infos,
        });
    }

    Ok(table_infos)
}
