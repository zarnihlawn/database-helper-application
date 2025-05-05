use mongodb::{
    bson::doc,
    options::{ClientOptions, ServerApi, ServerApiVersion},
    Client,
};

use crate::models::structs::erd_struct::CollectionInfo;

use super::app_database_connection::get_db_path;

use sqlx::sqlite::SqlitePool;

use futures_util::StreamExt;
use serde_json;

#[tauri::command]
pub async fn test_mongo_connection(url: String) -> Result<(), String> {
    let mut client_options = ClientOptions::parse(url).await.map_err(|e| e.to_string())?;

    // Set the server_api field of the client_options object to Stable API version 1
    let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
    client_options.server_api = Some(server_api);

    // Create a new client and connect to the server
    let client = Client::with_options(client_options).map_err(|e| e.to_string())?;

    // Send a ping to confirm a successful connection
    client
        .database("admin")
        .run_command(doc! { "ping": 1 })
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub async fn save_mongo_connection(
    user_id: Option<i64>,
    url: String,
    connection_name: String,
) -> Result<(), String> {
    test_mongo_connection(url.clone()).await.map_err(|e| e)?;

    let app_database = get_db_path();
    let pool = SqlitePool::connect(&format!("sqlite://{}", app_database))
        .await
        .unwrap();

    if let Some(uid) = user_id {
        // Insert with user_id
        sqlx::query("INSERT INTO database_connection (user_id, datasource_id, connection_name, url) VALUES (?, ?, ?, ?)")
            .bind(uid)
            .bind(3)
            .bind(connection_name.as_str())
            .bind(url.as_str())
            .execute(&pool)
            .await
            .map_err(|e| e.to_string())?;
    } else {
        // Insert without user_id
        sqlx::query("INSERT INTO database_connection (datasource_id, connection_name, url) VALUES (?, ?, ?)")
            .bind(3)
            .bind(connection_name.as_str())
            .bind(url.as_str())
            .execute(&pool)
            .await
            .map_err(|e| e.to_string())?;
    }

    Ok(())
}

#[tauri::command]
pub async fn get_database_from_mongo(url: String) -> Result<Vec<CollectionInfo>, String> {
    let mut client_options = ClientOptions::parse(url).await.map_err(|e| e.to_string())?;
    let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
    client_options.server_api = Some(server_api);
    let client = Client::with_options(client_options).map_err(|e| e.to_string())?;

    let mut collections = Vec::new();

    // Get all database names including system databases
    let db_names = client
        .list_database_names()
        .await
        .map_err(|e| e.to_string())?;

    // Process each database
    for db_name in db_names {
        let db = client.database(&db_name);

        // Get all collections including system collections
        let collection_names = db
            .list_collection_names()
            .await
            .map_err(|e| e.to_string())?;

        for collection_name in collection_names {
            collections.push(CollectionInfo {
                database_name: db_name.clone(),
                collection_name,
            });
        }
    }

    Ok(collections)
}

pub async fn run_query_block_mongo(
    url: String,
    content: String,
) -> Result<serde_json::Value, String> {
    let mut client_options = ClientOptions::parse(url).await.map_err(|e| e.to_string())?;
    let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
    client_options.server_api = Some(server_api);
    let client = Client::with_options(client_options).map_err(|e| e.to_string())?;

    // Check if the content is a MongoDB shell script (contains multiple lines with db.)
    if content.contains("db.")
        && content
            .lines()
            .filter(|line| line.trim().starts_with("db."))
            .count()
            > 0
    {
        // This is a shell script, we need to parse and execute each command separately
        let mut results = Vec::new();
        let mut current_db = "admin".to_string();

        // Split the content into lines and process each command
        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with("//") {
                continue; // Skip empty lines and comments
            }

            // Handle "use" command to switch database
            if line.starts_with("use ") {
                current_db = line[4..].trim().to_string();
                continue;
            }

            // Handle db. commands
            if line.starts_with("db.") {
                let command_part = &line[3..]; // Remove "db." prefix

                // Parse collection name and operation
                if let Some(dot_pos) = command_part.find('.') {
                    let collection_name = &command_part[..dot_pos];
                    let operation = &command_part[dot_pos + 1..];

                    // Handle different operations
                    if operation.starts_with("find(") {
                        // Extract find criteria
                        let criteria_str = operation[5..operation.len() - 1].trim();
                        let filter = if criteria_str.is_empty() {
                            doc! {}
                        } else {
                            // Try to parse the criteria as JSON
                            match serde_json::from_str::<serde_json::Value>(&format!(
                                "{{{}}}",
                                criteria_str
                            )) {
                                Ok(filter) => match mongodb::bson::to_bson(&filter) {
                                    Ok(bson_filter) => {
                                        match mongodb::bson::from_bson(bson_filter) {
                                            Ok(doc_filter) => doc_filter,
                                            Err(e) => {
                                                return Err(format!(
                                                    "Failed to convert BSON to Document: {}",
                                                    e
                                                ));
                                            }
                                        }
                                    }
                                    Err(e) => {
                                        return Err(format!(
                                            "Failed to convert filter to BSON: {}",
                                            e
                                        ));
                                    }
                                },
                                Err(e) => {
                                    doc! {} // Default to empty filter if parsing fails
                                }
                            }
                        };

                        // Execute find operation
                        let db = client.database(&current_db);
                        let collection = db.collection::<mongodb::bson::Document>(collection_name);
                        let mut cursor =
                            collection.find(filter).await.map_err(|e| e.to_string())?;

                        // Collect all documents
                        let mut documents = Vec::new();
                        while let Some(doc_result) = cursor.next().await {
                            let doc = doc_result.map_err(|e| e.to_string())?;
                            let doc_json = serde_json::to_value(doc).map_err(|e| {
                                format!("Failed to convert document to JSON: {}", e)
                            })?;
                            documents.push(doc_json);
                        }

                        results.push(serde_json::json!({
                            "operation": "find",
                            "collection": collection_name,
                            "results": documents
                        }));
                    } else if operation.starts_with("insertMany(") {
                        // Extract documents to insert
                        let docs_str = operation[11..operation.len() - 1].trim();
                        if docs_str.starts_with("[") && docs_str.ends_with("]") {
                            let docs_str = &docs_str[1..docs_str.len() - 1];
                            let docs_parts: Vec<&str> = docs_str.split("},").collect();

                            let mut documents = Vec::new();
                            for doc_part in docs_parts {
                                let doc_part = doc_part.trim();
                                if doc_part.is_empty() {
                                    continue;
                                }

                                let doc_str = if doc_part.ends_with("}") {
                                    doc_part.to_string()
                                } else {
                                    format!("{}}}", doc_part)
                                };

                                match serde_json::from_str::<serde_json::Value>(&doc_str) {
                                    Ok(doc) => {
                                        let bson_doc = mongodb::bson::from_bson(
                                            mongodb::bson::to_bson(&doc).map_err(|e| {
                                                format!("Failed to convert document to BSON: {}", e)
                                            })?,
                                        )
                                        .map_err(|e| {
                                            format!("Failed to convert BSON to Document: {}", e)
                                        })?;
                                        documents.push(bson_doc);
                                    }
                                    Err(e) => {
                                        return Err(format!("Failed to parse document: {}", e))
                                    }
                                }
                            }

                            // Execute insertMany operation
                            let db = client.database(&current_db);
                            let collection = db.collection(collection_name);
                            let result = collection
                                .insert_many(documents)
                                .await
                                .map_err(|e| e.to_string())?;

                            results.push(serde_json::json!({
                                "operation": "insertMany",
                                "collection": collection_name,
                                "inserted_count": result.inserted_ids.len()
                            }));
                        }
                    } else if operation.starts_with("createCollection(") {
                        // Extract collection name
                        let collection_name = operation[16..operation.len() - 1].trim();

                        // Execute createCollection operation
                        let db = client.database(&current_db);
                        let result = db
                            .create_collection(collection_name)
                            .await
                            .map_err(|e| e.to_string());

                        results.push(serde_json::json!({
                            "operation": "createCollection",
                            "collection": collection_name,
                            "success": result.is_ok()
                        }));
                    } else {
                        // For other operations, return a generic response
                        results.push(serde_json::json!({
                            "operation": "unknown",
                            "command": line,
                            "message": "Operation not fully supported"
                        }));
                    }
                }
            }
        }

        return Ok(serde_json::json!({ "results": results }));
    }

    // Parse the content as a MongoDB command
    let command: serde_json::Value = serde_json::from_str(&content)
        .map_err(|e| format!("Failed to parse MongoDB command: {}", e))?;

    // Extract database and collection names if they exist in the command
    let db_name = command
        .get("db")
        .and_then(|v| v.as_str())
        .unwrap_or("admin");

    let collection_name = command.get("collection").and_then(|v| v.as_str());

    let db = client.database(db_name);

    // Convert JSON value to MongoDB Document
    let command_doc = mongodb::bson::from_bson(
        mongodb::bson::to_bson(&command)
            .map_err(|e| format!("Failed to convert command to BSON: {}", e))?,
    )
    .map_err(|e| format!("Failed to convert BSON to Document: {}", e))?;

    // Execute the command
    let result = db
        .run_command(command_doc)
        .await
        .map_err(|e| e.to_string())?;

    // Convert the result to a JSON value
    let result_json = serde_json::to_value(result)
        .map_err(|e| format!("Failed to convert result to JSON: {}", e))?;

    // If this is a find operation, format the results in a more user-friendly way
    if let Some(cmd) = command.get("find") {
        if let Some(docs) = result_json.get("cursor").and_then(|c| c.get("firstBatch")) {
            // Extract field names from the first document if available
            let mut column_names = Vec::new();
            if let Some(first_doc) = docs.as_array().and_then(|arr| arr.first()) {
                if let Some(obj) = first_doc.as_object() {
                    for key in obj.keys() {
                        column_names.push(key.clone());
                    }
                }
            }

            // Format the results as an array of objects with consistent fields
            let mut formatted_results = Vec::new();

            if let Some(docs_array) = docs.as_array() {
                for doc in docs_array {
                    let mut row_data = serde_json::Map::new();

                    // If we have column names, ensure all documents have the same fields
                    if !column_names.is_empty() {
                        for column in &column_names {
                            let value = doc.get(column).unwrap_or(&serde_json::Value::Null);
                            row_data.insert(column.clone(), value.clone());
                        }
                    } else {
                        // If no column names, just use the document as is
                        if let Some(obj) = doc.as_object() {
                            for (key, value) in obj {
                                row_data.insert(key.clone(), value.clone());
                            }
                        }
                    }

                    formatted_results.push(serde_json::Value::Object(row_data));
                }
            }

            return Ok(serde_json::json!({ "results": formatted_results }));
        }
    }

    // For other commands, return the raw result
    Ok(serde_json::json!({ "result": result_json }))
}
