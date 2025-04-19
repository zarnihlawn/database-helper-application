use sqlx::SqlitePool;
use surrealdb::engine::remote::ws::{Ws, Wss};
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;

use url::Url;

use super::app_database_connection::get_db_path;

#[tauri::command]
pub async fn test_surrealdb_connection(url: String) -> Result<(), String> {
    let parsed_url = Url::parse(&url).map_err(|e| e.to_string())?;
    let scheme = parsed_url.scheme();
    if scheme != "ws" && scheme != "wss" {
        return Err("Only ws:// or wss:// SurrealDB URLs are supported".to_string());
    }

    let db = match scheme {
        "ws" => Surreal::new::<Ws>(&url[..]).await,
        "wss" => Surreal::new::<Wss>(&url[..]).await,
        _ => unreachable!(),
    }
    .map_err(|e| format!("Failed to connect: {e}"))?;

    let username = parsed_url.username();
    let password = parsed_url.password();

    if !username.is_empty() {
        let password = password.ok_or("Password is required for authentication")?;
        db.signin(Root { username, password })
            .await
            .map_err(|e| format!("Failed to sign in: {e}"))?;
    }

    db.use_ns("test")
        .use_db("test")
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub async fn save_surrealdb_connection(
    user_id: Option<i64>,
    url: String,
    connection_name: String,
) -> Result<(), String> {
    test_surrealdb_connection(url.clone())
        .await
        .map_err(|e| e)?;

    let app_database = get_db_path();
    let pool = SqlitePool::connect(&format!("sqlite://{}", app_database))
        .await
        .unwrap();

    if let Some(uid) = user_id {
        sqlx::query("INSERT INTO database_connection (user_id, datasource_id, connection_name, url) VALUES (?, ?, ?, ?)")
            .bind(uid)
            .bind(7)
            .bind(connection_name.as_str())
            .bind(url.as_str())
            .execute(&pool)
            .await
            .map_err(|e| e.to_string())?;
    } else {
        sqlx::query("INSERT INTO database_connection (datasource_id, connection_name, url) VALUES (?, ?, ?)")
            .bind(7)
            .bind(connection_name.as_str())
            .bind(url.as_str())
            .execute(&pool)
            .await
            .map_err(|e| e.to_string())?;
    }

    Ok(())
}
