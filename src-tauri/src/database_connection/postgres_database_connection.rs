use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPool, Row, SqlitePool};
use std::collections::HashMap;

use super::app_database_connection::get_db_path;

#[derive(Debug, Serialize, Deserialize)]
pub struct DatabaseObjects {
    tables: Vec<String>,
    views: Vec<String>,
    functions: Vec<String>,
    sequences: Vec<String>,
}

#[tauri::command]
pub async fn test_postgres_connection(url: String) -> Result<String, String> {
    let pool = PgPool::connect(&url).await.map_err(|e| e.to_string())?;

    let _ = sqlx::query("SELECT 1")
        .fetch_all(&pool)
        .await
        .map_err(|e| e.to_string())?;

    Ok("Connected".to_string())
}

#[tauri::command]
pub async fn save_postgres_connection(
    user_id: Option<i64>,
    url: String,
    connection_name: String,
) -> Result<(), String> {
    test_postgres_connection(url.clone()).await.map_err(|e| e)?;

    let app_database = get_db_path();
    let pool = SqlitePool::connect(&format!("sqlite://{}", app_database))
        .await
        .unwrap();

    if let Some(uid) = user_id {
        // Insert with user_id
        sqlx::query("INSERT INTO database_connection (user_id, datasource_id, connection_name, url) VALUES (?, ?, ?, ?)")
            .bind(uid)
            .bind(1)
            .bind(connection_name.as_str())
            .bind(url.as_str())
            .execute(&pool)
            .await
            .map_err(|e| e.to_string())?;
    } else {
        // Insert without user_id
        sqlx::query("INSERT INTO database_connection (datasource_id, connection_name, url) VALUES (?, ?, ?)")
            .bind(1)
            .bind(connection_name.as_str())
            .bind(url.as_str())
            .execute(&pool)
            .await
            .map_err(|e| e.to_string())?;
    }

    Ok(())
}

#[tauri::command]
pub async fn get_database_from_postgres(
    url: String,
) -> Result<HashMap<String, HashMap<String, DatabaseObjects>>, String> {
    let pool = PgPool::connect(&url).await.map_err(|e| e.to_string())?;

    let databases = sqlx::query("SELECT datname FROM pg_database")
        .fetch_all(&pool)
        .await
        .map_err(|e| e.to_string())?;

    let database_names: Vec<String> = databases
        .iter()
        .map(|row| row.get::<String, _>("datname"))
        .collect();

    let mut database_schemas = HashMap::new();
    for database in &database_names {
        let schemas = sqlx::query("SELECT schema_name FROM information_schema.schemata")
            .fetch_all(&pool)
            .await
            .map_err(|e| e.to_string())?;

        let mut schema_objects = HashMap::new();
        for schema in schemas {
            let schema_name: String = schema.get::<String, _>("schema_name");

            // Get tables - return empty vec if none found
            let tables = sqlx::query("SELECT table_name FROM information_schema.tables WHERE table_schema = $1 AND table_type = 'BASE TABLE'")
                .bind(&schema_name)
                .fetch_all(&pool)
                .await
                .unwrap_or_default();
            let table_names: Vec<String> = tables
                .iter()
                .map(|row| row.get::<String, _>("table_name"))
                .collect();

            // Get views - return empty vec if none found
            let views = sqlx::query(
                "SELECT table_name FROM information_schema.views WHERE table_schema = $1",
            )
            .bind(&schema_name)
            .fetch_all(&pool)
            .await
            .unwrap_or_default();
            let view_names: Vec<String> = views
                .iter()
                .map(|row| row.get::<String, _>("table_name"))
                .collect();

            // Get functions - return empty vec if none found
            let functions = sqlx::query(
                "SELECT routine_name FROM information_schema.routines WHERE routine_schema = $1",
            )
            .bind(&schema_name)
            .fetch_all(&pool)
            .await
            .unwrap_or_default();
            let function_names: Vec<String> = functions
                .iter()
                .map(|row| row.get::<String, _>("routine_name"))
                .collect();

            // Get sequences - return empty vec if none found
            let sequences = sqlx::query(
                "SELECT sequence_name FROM information_schema.sequences WHERE sequence_schema = $1",
            )
            .bind(&schema_name)
            .fetch_all(&pool)
            .await
            .unwrap_or_default();
            let sequence_names: Vec<String> = sequences
                .iter()
                .map(|row| row.get::<String, _>("sequence_name"))
                .collect();

            schema_objects.insert(
                schema_name,
                DatabaseObjects {
                    tables: table_names,
                    views: view_names,
                    functions: function_names,
                    sequences: sequence_names,
                },
            );
        }

        database_schemas.insert(database.clone(), schema_objects);
    }

    Ok(database_schemas)
}

pub async fn run_query_block_postgresql(url: String, content: String) -> Result<(), String> {
    let pool = PgPool::connect(&url).await.map_err(|e| e.to_string())?;

    let result = sqlx::query(&content)
        .fetch_all(&pool)
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}
