use oracle::Connection;
use sqlite::open;

use super::app_database_connection::get_db_path;

#[tauri::command]
pub async fn test_oracle_connection(url: String) -> Result<(), String> {
    // Parse the URL to extract connection details
    // Format: oracle://username:password@host:port/service_name
    let url_parts: Vec<&str> = url.split("://").collect();
    if url_parts.len() != 2 {
        return Err("Invalid Oracle connection URL format".to_string());
    }

    let credentials_and_host: Vec<&str> = url_parts[1].split('@').collect();
    if credentials_and_host.len() != 2 {
        return Err("Invalid Oracle connection URL format".to_string());
    }

    let credentials: Vec<&str> = credentials_and_host[0].split(':').collect();
    if credentials.len() != 2 {
        return Err("Invalid Oracle connection URL format".to_string());
    }

    let username = credentials[0];
    let password = credentials[1];
    let host_and_service = credentials_and_host[1];

    println!("username: {}", username);
    println!("password: {}", password);
    println!("host_and_service: {}", host_and_service);

    // Try to establish connection
    match Connection::connect(username, password, host_and_service) {
        Ok(conn) => {
            // Test the connection with a simple query
            match conn.query_row_as::<i32>("SELECT 1 FROM DUAL", &[]) {
                Ok(_) => {
                    println!("Successfully connected to Oracle database!");
                    Ok(())
                }
                Err(e) => Err(format!("Failed to execute test query: {}", e.to_string())),
            }
        }
        Err(e) => Err(format!(
            "Failed to connect to Oracle database: {}",
            e.to_string()
        )),
    }
}

#[tauri::command]
pub async fn save_oracle_connection(
    user_id: Option<i64>,
    url: String,
    connection_name: String,
) -> Result<(), String> {
    test_oracle_connection(url.clone()).await?;

    let app_database = get_db_path();
    let connection = open(app_database).map_err(|e| e.to_string())?;

    if let Some(uid) = user_id {
        // Insert with user_id
        let mut statement = connection
            .prepare("INSERT INTO database_connection (user_id, datasource_id, connection_name, url) VALUES (?, ?, ?, ?)")
            .map_err(|e| e.to_string())?;
        statement.bind((1, uid)).map_err(|e| e.to_string())?;
        statement.bind((2, 4)).map_err(|e| e.to_string())?;
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
        statement.bind((1, 4)).map_err(|e| e.to_string())?;
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
pub async fn get_database_from_oracle(url: String) -> Result<Vec<String>, String> {
    // Parse the URL to extract connection details
    let url_parts: Vec<&str> = url.split("://").collect();
    if url_parts.len() != 2 {
        return Err("Invalid Oracle connection URL format".to_string());
    }

    let credentials_and_host: Vec<&str> = url_parts[1].split('@').collect();
    if credentials_and_host.len() != 2 {
        return Err("Invalid Oracle connection URL format".to_string());
    }

    let credentials: Vec<&str> = credentials_and_host[0].split(':').collect();
    if credentials.len() != 2 {
        return Err("Invalid Oracle connection URL format".to_string());
    }

    let username = credentials[0];
    let password = credentials[1];
    let host_and_service = credentials_and_host[1];

    // Connect to Oracle
    let conn = Connection::connect(username, password, host_and_service)
        .map_err(|e| format!("Failed to connect to Oracle: {}", e))?;

    // Get list of tables
    let mut tables = Vec::new();
    let rows = conn
        .query("SELECT table_name FROM user_tables", &[])
        .map_err(|e| format!("Failed to query tables: {}", e))?;

    for row_result in rows {
        let row = row_result.map_err(|e| format!("Failed to read row: {}", e))?;
        let table_name: String = row
            .get(0)
            .map_err(|e| format!("Failed to get table name: {}", e))?;
        tables.push(table_name);
    }

    Ok(tables)
}
