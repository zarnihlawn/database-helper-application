use serde::{Deserialize, Serialize};
use serde_json;
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

#[tauri::command]
pub async fn run_query_block_mysql(
    url: String,
    content: String,
) -> Result<serde_json::Value, String> {
    let pool = MySqlPool::connect(&url).await.map_err(|e| e.to_string())?;

    // Determine if the query is a SELECT query
    let is_select = content.trim().to_uppercase().starts_with("SELECT");

    if is_select {
        // Extract column names from the query
        let mut column_names = Vec::new();

        // Check if the query uses SELECT *
        let is_select_all = content.trim().to_uppercase().contains("SELECT *");

        if is_select_all {
            // For SELECT * queries, we need to extract the table name and get its schema
            if let Some(from_part) = content.split_once("FROM") {
                let table_part = from_part.1.trim();

                // Extract the table name (handle JOINs, WHERE, etc.)
                let table_name = if let Some(where_pos) = table_part.find("WHERE") {
                    table_part[..where_pos].trim()
                } else if let Some(join_pos) = table_part.find("JOIN") {
                    table_part[..join_pos].trim()
                } else {
                    table_part
                };

                // Remove any table alias
                let table_name = if let Some(alias_pos) = table_name.find(" AS ") {
                    table_name[..alias_pos].trim()
                } else {
                    table_name
                };

                // Extract schema and table if specified as schema.table
                let (schema, table) = if let Some(dot_pos) = table_name.find('.') {
                    let schema = table_name[..dot_pos].trim();
                    let table = table_name[dot_pos + 1..].trim();
                    (Some(schema), table)
                } else {
                    (None, table_name)
                };

                // Get column names from the table schema
                let schema_query = if let Some(schema_name) = schema {
                    format!(
                        "SELECT COLUMN_NAME FROM information_schema.COLUMNS 
                         WHERE TABLE_SCHEMA = '{}' AND TABLE_NAME = '{}' 
                         ORDER BY ORDINAL_POSITION",
                        schema_name, table
                    )
                } else {
                    format!(
                        "SELECT COLUMN_NAME FROM information_schema.COLUMNS 
                         WHERE TABLE_NAME = '{}' 
                         ORDER BY ORDINAL_POSITION",
                        table
                    )
                };

                let schema_rows = sqlx::query(&schema_query)
                    .fetch_all(&pool)
                    .await
                    .map_err(|e| e.to_string())?;

                for row in schema_rows {
                    if let Ok(name) = row.try_get::<String, _>("COLUMN_NAME") {
                        column_names.push(name);
                    }
                }
            }
        } else {
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
        }

        // For SELECT queries, fetch and return the results
        let rows = sqlx::query(content.as_str())
            .fetch_all(&pool)
            .await
            .map_err(|e| e.to_string())?;

        let mut results = Vec::new();

        // Get column count from the first row if available
        let column_count = if !rows.is_empty() { rows[0].len() } else { 0 };

        // If we couldn't extract column names from the query, use generic names
        if column_names.is_empty() && column_count > 0 {
            for i in 0..column_count {
                column_names.push(format!("column{}", i));
            }
        }

        for row in rows {
            let mut row_data = serde_json::Map::new();

            // Add each column value to the result
            for i in 0..column_count {
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
                    // If all else fails, try to get as JSON value
                    match row.try_get::<serde_json::Value, _>(i) {
                        Ok(v) => v,
                        Err(_) => serde_json::Value::Null,
                    }
                };

                row_data.insert(column_name, value);
            }

            results.push(serde_json::Value::Object(row_data));
        }

        Ok(serde_json::json!({ "results": results }))
    } else {
        // For non-SELECT queries (INSERT, UPDATE, DELETE, etc.), execute and return affected rows
        let result = sqlx::query(content.as_str())
            .execute(&pool)
            .await
            .map_err(|e| e.to_string())?;

        Ok(serde_json::json!({
            "affected_rows": result.rows_affected()
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
