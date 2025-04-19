use super::app_database_connection::get_db_path;
use crate::models::structs::erd_struct::{ColumnInfo, TableInfo};
use sqlx::sqlite::SqlitePool;
use tiberius::{error::Error, AuthMethod, Client, Config};
use tokio::net::TcpStream;
use tokio_util::compat::TokioAsyncWriteCompatExt;
use url::Url;

#[tauri::command]
pub async fn test_mssql_connection(url: String) -> Result<String, String> {
    let (host, port, username, password) = parse_mssql_url(&url)?;

    let mut config = Config::new();
    config.host(&host);
    config.port(port);
    config.authentication(AuthMethod::sql_server(&username, &password));
    config.encryption(tiberius::EncryptionLevel::NotSupported);

    let tcp = TcpStream::connect(config.get_addr())
        .await
        .map_err(|e| e.to_string())?;
    tcp.set_nodelay(true).map_err(|e| e.to_string())?;

    match Client::connect(config, tcp.compat_write()).await {
        Ok(_) => Ok("Connected".to_string()),
        Err(Error::Routing { host, port }) => {
            let mut config = Config::new();
            config.host(&host);
            config.port(port);
            config.authentication(AuthMethod::sql_server(&username, &password));

            let tcp = TcpStream::connect(config.get_addr())
                .await
                .map_err(|e| e.to_string())?;
            tcp.set_nodelay(true).map_err(|e| e.to_string())?;

            Client::connect(config, tcp.compat_write())
                .await
                .map(|_| "Connected".to_string())
                .map_err(|e| e.to_string())
        }
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub async fn save_mssql_connection(
    user_id: Option<i64>,
    url: String,
    connection_name: String,
) -> Result<(), String> {
    // Test connection first
    test_mssql_connection(url.clone()).await?;

    // Save to local database
    let app_database = get_db_path();
    let pool = SqlitePool::connect(&format!("sqlite://{}", app_database))
        .await
        .map_err(|e| e.to_string())?;

    if let Some(uid) = user_id {
        sqlx::query(
            "INSERT INTO database_connection (user_id, datasource_id, connection_name, url) VALUES (?, ?, ?, ?)",
        )
        .bind(uid)
        .bind(5) // MSSQL datasource_id
        .bind(connection_name)
        .bind(url)
        .execute(&pool)
        .await
        .map_err(|e| e.to_string())?;
    } else {
        sqlx::query(
            "INSERT INTO database_connection (datasource_id, connection_name, url) VALUES (?, ?, ?)",
        )
        .bind(5) // MSSQL datasource_id
        .bind(connection_name)
        .bind(url)
        .execute(&pool)
        .await
        .map_err(|e| e.to_string())?;
    }

    Ok(())
}

fn parse_mssql_url(url: &str) -> Result<(String, u16, String, String), String> {
    let parsed_url = Url::parse(url).map_err(|e| e.to_string())?;

    let host = parsed_url
        .host_str()
        .ok_or("Invalid host in URL")?
        .to_string();

    let port = parsed_url.port().unwrap_or(1433);

    let username = parsed_url.username().to_string();
    if username.is_empty() {
        return Err("Username is required".to_string());
    }

    let password = parsed_url
        .password()
        .ok_or("Password is required")?
        .to_string();

    Ok((host, port, username, password))
}

#[tauri::command]
pub async fn get_database_from_mssql(url: String) -> Result<Vec<TableInfo>, String> {
    let (host, port, username, password) = parse_mssql_url(&url)?;

    // Create MSSQL connection
    let mut config = Config::new();
    config.host(&host);
    config.port(port);
    config.authentication(AuthMethod::sql_server(&username, &password));
    config.encryption(tiberius::EncryptionLevel::NotSupported);

    let tcp = TcpStream::connect(config.get_addr())
        .await
        .map_err(|e| e.to_string())?;
    tcp.set_nodelay(true).map_err(|e| e.to_string())?;

    let mut client = Client::connect(config, tcp.compat_write())
        .await
        .map_err(|e| e.to_string())?;

    // Get all tables
    let mut tables = Vec::new();
    let rows = client
        .query(
            "SELECT TABLE_SCHEMA, TABLE_NAME FROM INFORMATION_SCHEMA.TABLES WHERE TABLE_TYPE = 'BASE TABLE'",
            &[],
        )
        .await
        .map_err(|e| e.to_string())?
        .into_first_result()
        .await
        .map_err(|e| e.to_string())?;

    for row in rows {
        let schema: &str = row.get(0).ok_or("Failed to get schema")?;
        let table_name: &str = row.get(1).ok_or("Failed to get table name")?;

        // Get columns for each table
        let mut columns = Vec::new();
        let col_rows = client
            .query(
                &format!(
                    "SELECT COLUMN_NAME, DATA_TYPE, CHARACTER_MAXIMUM_LENGTH 
                     FROM INFORMATION_SCHEMA.COLUMNS 
                     WHERE TABLE_SCHEMA = '{}' AND TABLE_NAME = '{}'",
                    schema, table_name
                ),
                &[],
            )
            .await
            .map_err(|e| e.to_string())?
            .into_first_result()
            .await
            .map_err(|e| e.to_string())?;

        for col_row in col_rows {
            let column_name: &str = col_row.get(0).ok_or("Failed to get column name")?;
            let data_type: &str = col_row.get(1).ok_or("Failed to get data type")?;
            let max_length: Option<i32> = col_row.get(2);

            let data_type_with_length = if let Some(len) = max_length {
                format!("{}({})", data_type, len)
            } else {
                data_type.to_string()
            };

            columns.push(ColumnInfo {
                name: column_name.to_string(),
                data_type: data_type_with_length,
            });
        }

        tables.push(TableInfo {
            name: format!("{}.{}", schema, table_name),
            columns,
        });
    }

    Ok(tables)
}
