// use dotenv::dotenv;
// use sqlite::{open, State};
// use std::env::var;

// #[tauri::command]
// pub async fn test_sqlite_connection(url: String) -> Result<String, String> {
//     let database_url = url;

//     let connection = open(database_url).map_err(|e| format!("Failed to open database: {}", e))?;

//     let mut statement = connection
//         .prepare("SELECT 1")
//         .map_err(|e| format!("Failed to prepare statement: {}", e))?;
//     match statement.next() {
//         Ok(State::Done) => Ok("Connected".to_string()),
//         Ok(State::Row) => Ok("Connected".to_string()), // Connection successful if we get a row
//         Err(e) => Err(format!("Failed to execute statement: {}", e)),
//     }
// }

// #[tauri::command]
// pub async fn save_sqlite_connection(url: String) -> Result<(), String> {
//     dotenv().ok();
//     let app_database = var("DATABASE_URL").map_err(|e| e.to_string())?;
//     let database_url = url.clone();

//     let result = test_sqlite_connection(url).await;

//     if result.is_ok() {
//         let connection =
//             open(app_database).map_err(|e| format!("Failed to open database: {}", e))?;
//         let query = "INSERT INTO database_connection (user_id, datasource_id, connection_name, url) VALUES (?, ?, ?, ?)";
//         let mut statement = connection
//             .prepare(query)
//             .map_err(|e| format!("Failed to prepare statement: {}", e))?;
//         statement
//             .bind((1, 1))
//             .map_err(|e| format!("Failed to bind parameter: {}", e))?;
//         statement
//             .bind((2, 2))
//             .map_err(|e| format!("Failed to bind parameter: {}", e))?;
//         statement
//             .bind((3, "Sqlite Connection 1"))
//             .map_err(|e| format!("Failed to bind parameter: {}", e))?;
//         statement
//             .bind((4, database_url.as_str()))
//             .map_err(|e| format!("Failed to bind parameter: {}", e))?;
//         if let State::Done = statement.next().map_err(|e| e.to_string())? {
//             Ok(())
//         } else {
//             Err("Failed to execute statement".to_string())
//         }
//     } else {
//         Err(result.unwrap_err())
//     }
// }
