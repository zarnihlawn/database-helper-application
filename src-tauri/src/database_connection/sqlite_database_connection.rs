use crate::database_connection::app_database_connection::get_db_path;

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
    connection_name: String,
    url: String,
) -> Result<(), String> {
    // Only insert if the connection test succeeds
    println!("Connecting Rust");
    test_sqlite_connection(url.clone()).await.map_err(|e| e)?;

    let app_database = get_db_path();
    let connection = open(app_database).map_err(|e| e.to_string())?;

    if let Some(uid) = user_id {
        // Insert with user_id
        println!("Connecting with the user: {:?}", user_id);
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
        println!("Connecting without user_name");
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
