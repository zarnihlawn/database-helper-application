use mongodb::{
    bson::doc,
    options::{ClientOptions, ServerApi, ServerApiVersion},
    Client,
};
use sqlite::open;

use crate::models::structs::erd_struct::CollectionInfo;

use super::app_database_connection::get_db_path;

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
    println!("Pinged your deployment. You successfully connected to MongoDB!");

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
    let connection = open(app_database).map_err(|e| e.to_string())?;

    if let Some(uid) = user_id {
        // Insert with user_id
        let mut statement = connection
            .prepare("INSERT INTO database_connection (user_id, datasource_id, connection_name, url) VALUES (?, ?, ?, ?)")
            .map_err(|e| e.to_string())?;
        statement.bind((1, uid)).map_err(|e| e.to_string())?;
        statement.bind((2, 3)).map_err(|e| e.to_string())?;
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
        statement.bind((1, 3)).map_err(|e| e.to_string())?;
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
