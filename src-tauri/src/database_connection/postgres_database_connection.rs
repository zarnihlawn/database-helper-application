use super::app_database_connection::get_db_path;
use crate::models::structs::erd_struct::{ColumnInfo, TableInfo};
use sqlite::open;
use postgres::{Client, NoTls};
use postgres::Error as PostgresError;
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};


#[tauri::command]
pub async fn test_postgres_connection(url: String) -> Result<String, String> {
    Ok(("Connected").to_string())
}

#[tauri::command]
pub async fn save_postgres_connection(
    user_id: Option<i64>,
    url: String,
    connection_name: String,
) -> Result<(), String> {
    // Test the connection first
    test_postgres_connection(url.clone()).await.map_err(|e| e)?;

    // Save to our app database
    let app_database = get_db_path();
    let connection = open(app_database).map_err(|e| e.to_string())?;

    if let Some(uid) = user_id {
        // Insert with user_id
        let mut statement = connection
            .prepare("INSERT INTO database_connection (user_id, datasource_id, connection_name, url) VALUES (?, ?, ?, ?)")
            .map_err(|e| e.to_string())?;
        statement.bind((1, uid)).map_err(|e| e.to_string())?;
        statement.bind((2, 1)).map_err(|e| e.to_string())?;
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
        statement.bind((1, 1)).map_err(|e| e.to_string())?;
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
pub async fn get_database_from_postgres(url: String) -> Result<Vec<TableInfo>, String> {
    Ok(vec![])
}
