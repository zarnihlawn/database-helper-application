use super::app_database_connection::get_db_path;
use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqlitePool;
use tiberius::{error::Error, AuthMethod, Client, Config};
use tokio::net::TcpStream;
use tokio_util::compat::TokioAsyncWriteCompatExt;
use url::Url;

#[derive(Debug, Serialize, Deserialize)]
pub struct ColumnInfo {
    name: String,
    data_type: String,
    is_nullable: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TableInfo {
    name: String,
    columns: Vec<ColumnInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SchemaInfo {
    name: String,
    tables: Vec<TableInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DatabaseInfo {
    name: String,
    schemas: Vec<SchemaInfo>,
}

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
pub async fn get_database_from_mssql(url: String) -> Result<Vec<DatabaseInfo>, String> {
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

    // Get all databases (including system databases)
    let databases = client
        .query("SELECT name FROM sys.databases WHERE state = 0", &[]) // state = 0 means online databases
        .await
        .map_err(|e| e.to_string())?
        .into_first_result()
        .await
        .map_err(|e| e.to_string())?;

    let mut database_infos = Vec::new();
    for db_row in databases {
        let db_name: &str = db_row.get(0).ok_or("Failed to get database name")?;

        // Switch to this database
        client
            .query(&format!("USE [{}]", db_name), &[])
            .await
            .map_err(|e| e.to_string())?;

        // Get all schemas (including system schemas)
        let schemas = client
            .query("SELECT name FROM sys.schemas", &[])
            .await
            .map_err(|e| e.to_string())?
            .into_first_result()
            .await
            .map_err(|e| e.to_string())?;

        let mut schema_infos = Vec::new();
        for schema_row in schemas {
            let schema_name: &str = schema_row.get(0).ok_or("Failed to get schema name")?;

            // Get all tables for this schema
            let tables = client
                .query(
                    &format!(
                        "SELECT t.name 
                         FROM sys.tables t 
                         INNER JOIN sys.schemas s ON t.schema_id = s.schema_id 
                         WHERE s.name = '{}'",
                        schema_name
                    ),
                    &[],
                )
                .await
                .map_err(|e| e.to_string())?
                .into_first_result()
                .await
                .map_err(|e| e.to_string())?;

            let mut table_infos = Vec::new();
            for table_row in tables {
                let table_name: &str = table_row.get(0).ok_or("Failed to get table name")?;

                // Get all columns for this table
                let columns = client
                    .query(
                        &format!(
                            "SELECT c.name, t.name as data_type, c.is_nullable
                             FROM sys.columns c
                             INNER JOIN sys.types t ON c.user_type_id = t.user_type_id
                             WHERE OBJECT_ID = OBJECT_ID('{}.{}')",
                            schema_name, table_name
                        ),
                        &[],
                    )
                    .await
                    .map_err(|e| e.to_string())?
                    .into_first_result()
                    .await
                    .map_err(|e| e.to_string())?;

                let mut column_infos = Vec::new();
                for col_row in columns {
                    let column_name: &str = col_row.get(0).ok_or("Failed to get column name")?;
                    let data_type: &str = col_row.get(1).ok_or("Failed to get data type")?;
                    let is_nullable: bool = col_row.get(2).ok_or("Failed to get is_nullable")?;

                    column_infos.push(ColumnInfo {
                        name: column_name.to_string(),
                        data_type: data_type.to_string(),
                        is_nullable: if is_nullable { "YES" } else { "NO" }.to_string(),
                    });
                }

                table_infos.push(TableInfo {
                    name: table_name.to_string(),
                    columns: column_infos,
                });
            }

            schema_infos.push(SchemaInfo {
                name: schema_name.to_string(),
                tables: table_infos,
            });
        }

        database_infos.push(DatabaseInfo {
            name: db_name.to_string(),
            schemas: schema_infos,
        });
    }

    Ok(database_infos)
}

#[tauri::command]
pub async fn run_query_block_mssql(url: String, content: String) -> Result<(), String> {
    let (host, port, username, password) = parse_mssql_url(&url)?;

    let mut config = Config::new();
    config.host(&host);
    config.port(port);
    config.authentication(AuthMethod::sql_server(&username, &password));

    let tcp = TcpStream::connect(config.get_addr())
        .await
        .map_err(|e| e.to_string())?;
    tcp.set_nodelay(true).map_err(|e| e.to_string())?;

    let mut client = Client::connect(config, tcp.compat_write())
        .await
        .map_err(|e| e.to_string())?;

    // Execute the content query directly
    client
        .query(content.as_str(), &[])
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}
