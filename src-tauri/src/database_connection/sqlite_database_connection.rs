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

#[tauri::command]
pub async fn run_query_block_sqlite(
    url: String,
    content: String,
) -> Result<serde_json::Value, String> {
    let pool = SqlitePool::connect(&format!("sqlite://{}", url))
        .await
        .map_err(|e| e.to_string())?;

    // Determine if the query is a SELECT query
    let is_select = content.trim().to_uppercase().starts_with("SELECT");

    if is_select {
        // First, execute a query to get column names
        // We'll use a subquery with LIMIT 0 to get the column structure without data
        let column_query = format!("SELECT * FROM ({}) LIMIT 0", content);

        // Execute the query to get column information
        let columns = sqlx::query(&column_query)
            .fetch_all(&pool)
            .await
            .map_err(|e| e.to_string())?;

        // Extract column names from the query
        let mut column_names = Vec::new();

        // Try to extract column names from the query itself
        if let Some(select_part) = content.split_once("FROM") {
            let columns_part = select_part.0.trim();

            // Remove the SELECT keyword
            let columns_part = columns_part.trim_start_matches("SELECT").trim();

            // Split by commas, handling potential subqueries
            let mut in_parentheses = 0;
            let mut current_column = String::new();

            for c in columns_part.chars() {
                if c == '(' {
                    in_parentheses += 1;
                    current_column.push(c);
                } else if c == ')' {
                    in_parentheses -= 1;
                    current_column.push(c);
                } else if c == ',' && in_parentheses == 0 {
                    // Process the current column
                    let column = process_column_name(&current_column);
                    column_names.push(column);
                    current_column.clear();
                } else {
                    current_column.push(c);
                }
            }

            // Process the last column
            if !current_column.is_empty() {
                let column = process_column_name(&current_column);
                column_names.push(column);
            }
        }

        // If we couldn't extract column names from the query, use generic names
        if column_names.is_empty() && !columns.is_empty() {
            for i in 0..columns[0].len() {
                column_names.push(format!("column{}", i));
            }
        }

        // Now execute the actual query to get data
        let rows = sqlx::query(&content)
            .fetch_all(&pool)
            .await
            .map_err(|e| e.to_string())?;

        let mut results = Vec::new();

        for row in rows {
            let mut row_data = serde_json::Map::new();

            // Add each column value to the result
            for i in 0..row.len() {
                let column_name = if i < column_names.len() {
                    column_names[i].clone()
                } else {
                    format!("column{}", i)
                };

                // Try different types to extract the value
                let value = if let Ok(v) = row.try_get::<i64, _>(i) {
                    serde_json::Value::Number(serde_json::Number::from(v))
                } else if let Ok(v) = row.try_get::<f64, _>(i) {
                    serde_json::Value::Number(
                        serde_json::Number::from_f64(v).unwrap_or(serde_json::Number::from(0)),
                    )
                } else if let Ok(v) = row.try_get::<String, _>(i) {
                    serde_json::Value::String(v)
                } else if let Ok(v) = row.try_get::<bool, _>(i) {
                    serde_json::Value::Bool(v)
                } else {
                    serde_json::Value::Null
                };

                row_data.insert(column_name, value);
            }

            results.push(serde_json::Value::Object(row_data));
        }

        Ok(serde_json::json!({ "results": results }))
    } else {
        // For non-SELECT queries (INSERT, UPDATE, DELETE, etc.), execute and return affected rows
        let result = sqlx::query(&content)
            .execute(&pool)
            .await
            .map_err(|e| e.to_string())?;

        Ok(serde_json::json!({
            "affected_rows": result.rows_affected(),
            "last_insert_id": result.last_insert_rowid()
        }))
    }
}

// Helper function to process a column name from the SELECT query
fn process_column_name(column: &str) -> String {
    let column = column.trim();

    // Handle "column AS alias" syntax
    if let Some(alias) = column.split_once(" AS ") {
        return alias.1.trim().to_string();
    }

    // Handle "table.column" syntax
    if let Some(dot_pos) = column.find('.') {
        return column[dot_pos + 1..].trim().to_string();
    }

    // Handle functions like COUNT(*), SUM(column), etc.
    if column.contains('(') {
        // Extract the function name
        let func_name = column.split('(').next().unwrap_or("").trim();
        return func_name.to_string();
    }

    // Default case: use the column as is
    column.to_string()
}

#[tauri::command]
pub async fn get_er_diagram_from_sqlite(url: String) -> Result<String, String> {
    let pool = SqlitePool::connect(&format!("sqlite://{}", url))
        .await
        .map_err(|e| e.to_string())?;

    // Get all tables
    let tables = sqlx::query("SELECT name FROM sqlite_master WHERE type='table'")
        .fetch_all(&pool)
        .await
        .map_err(|e| e.to_string())?;

    let mut er_data = Vec::new();

    for table in tables {
        let table_name: String = table.get(0);

        // Skip SQLite internal tables
        if table_name.starts_with("sqlite_") {
            continue;
        }

        // Get columns for each table
        let columns = sqlx::query(&format!("PRAGMA table_info('{}')", table_name))
            .fetch_all(&pool)
            .await
            .map_err(|e| e.to_string())?;

        // Get foreign keys for this table
        let foreign_keys = sqlx::query(&format!("PRAGMA foreign_key_list('{}')", table_name))
            .fetch_all(&pool)
            .await
            .map_err(|e| e.to_string())?;

        let mut column_data = Vec::new();

        // Process columns
        for column in columns {
            let column_name: String = column.get(1);
            let column_type: String = column.get(2);
            let not_null: i64 = column.get(3);
            let default_value: Option<String> = column.try_get(4).unwrap_or(None);
            let is_pk: i64 = column.get(5);

            // Check if this column is a foreign key
            let mut is_fk = false;
            let mut references = String::new();

            for fk in &foreign_keys {
                let fk_column: String = fk.get(3);
                if fk_column == column_name {
                    is_fk = true;
                    let ref_table: String = fk.get(2);
                    let ref_column: String = fk.get(4);
                    references = format!("{}({})", ref_table, ref_column);
                    break;
                }
            }

            // Build column object
            let mut column_obj = serde_json::Map::new();
            column_obj.insert(
                "columnName".to_string(),
                serde_json::Value::String(column_name),
            );
            column_obj.insert(
                "columnType".to_string(),
                serde_json::Value::String(column_type),
            );
            column_obj.insert(
                "isNullable".to_string(),
                serde_json::Value::Bool(not_null == 0),
            );

            if is_pk == 1 {
                column_obj.insert("isPrimaryKey".to_string(), serde_json::Value::Bool(true));
            }

            if is_fk {
                column_obj.insert("isForeignKey".to_string(), serde_json::Value::Bool(true));
                column_obj.insert(
                    "references".to_string(),
                    serde_json::Value::String(references),
                );
            }

            if let Some(default_val) = default_value {
                column_obj.insert(
                    "default".to_string(),
                    serde_json::Value::String(default_val),
                );
            }

            column_data.push(serde_json::Value::Object(column_obj));
        }

        // Build table object
        let mut table_obj = serde_json::Map::new();
        table_obj.insert(
            "tableName".to_string(),
            serde_json::Value::String(table_name),
        );
        table_obj.insert("columns".to_string(), serde_json::Value::Array(column_data));

        er_data.push(serde_json::Value::Object(table_obj));
    }

    // Convert to JSON string
    let json_data = serde_json::to_string(&er_data).map_err(|e| e.to_string())?;

    Ok(json_data)
}
