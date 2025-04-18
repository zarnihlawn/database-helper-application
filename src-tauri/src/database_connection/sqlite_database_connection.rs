use crate::database_connection::app_database_connection::get_db_path;
use crate::models::structs::erd_struct::{ColumnInfo, TableInfo};

use sqlite::{open, State};

#[tauri::command]
pub async fn test_sqlite_connection(url: String) -> Result<String, String> {
    let database_url = url;

    let connection = open(database_url).map_err(|e| format!("Failed to open database: {}", e))?;

    let mut statement = connection
        .prepare("SELECT 1")
        .map_err(|e| format!("Failed to prepare statement: {}", e))?;
    match statement.next() {
        Ok(State::Done) => Ok("Connected".to_string()),
        Ok(State::Row) => Ok("Connected".to_string()), // Connection successful if we get a row
        Err(e) => Err(format!("Failed to execute statement: {}", e)),
    }
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
    let connection = open(app_database).map_err(|e| e.to_string())?;

    if let Some(uid) = user_id {
        // Insert with user_id
        let mut statement = connection
            .prepare("INSERT INTO database_connection (user_id, datasource_id, connection_name, url) VALUES (?, ?, ?, ?)")
            .map_err(|e| e.to_string())?;
        statement.bind((1, uid)).map_err(|e| e.to_string())?;
        statement.bind((2, 2)).map_err(|e| e.to_string())?;
        statement
            .bind((3, connection_name.as_str()))
            .map_err(|e| e.to_string())?;
        statement
            .bind((4, url.as_str()))
            .map_err(|e| e.to_string())?;
        statement.next().map_err(|e| e.to_string())?;
    } else {
        // Insert without user_id
        let mut statement = connection
            .prepare("INSERT INTO database_connection (datasource_id, connection_name, url) VALUES (?, ?, ?)")
            .map_err(|e| e.to_string())?;
        statement.bind((1, 2)).map_err(|e| e.to_string())?;
        statement
            .bind((2, connection_name.as_str()))
            .map_err(|e| e.to_string())?;
        statement
            .bind((3, url.as_str()))
            .map_err(|e| e.to_string())?;
        statement.next().map_err(|e| e.to_string())?;
    }

    Ok(())
}

#[tauri::command]
pub async fn get_database_from_sqlite(url: String) -> Result<Vec<TableInfo>, String> {
    let connection = open(url).map_err(|e| e.to_string())?;
    let mut tables_statement = connection
        .prepare("SELECT name FROM sqlite_master WHERE type='table'")
        .map_err(|e| format!("Failed to prepare statement to get tables: {}", e))?;

    let mut table_infos = Vec::new();
    while let Ok(State::Row) = tables_statement.next() {
        let table_name = tables_statement
            .read::<String, _>(0)
            .map_err(|e| format!("Failed to get table name: {}", e))?;

        let mut columns_statement = connection
            .prepare(&format!("PRAGMA table_info('{}')", table_name))
            .map_err(|e| {
                format!(
                    "Failed to prepare statement to get columns for table '{}': {}",
                    table_name, e
                )
            })?;

        let mut columns = Vec::new();
        while let Ok(State::Row) = columns_statement.next() {
            let column_name = columns_statement.read::<String, _>(1).map_err(|e| {
                format!(
                    "Failed to get column name for table '{}': {}",
                    table_name, e
                )
            })?;
            let column_type = columns_statement.read::<String, _>(2).map_err(|e| {
                format!(
                    "Failed to get column type for table '{}': {}",
                    table_name, e
                )
            })?;
            columns.push(ColumnInfo {
                name: column_name,
                data_type: column_type,
            });
        }

        table_infos.push(TableInfo {
            name: table_name,
            columns,
        });
    }

    Ok(table_infos)
}
