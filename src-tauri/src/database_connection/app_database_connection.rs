use crate::models::structs::schema_struct::{
    ContentTypeStruct, DatabaseConnectionStruct, DatasourceStruct, QueryBlockStruct,
    QueryFileStruct, User,
};
use sqlx::sqlite::SqlitePool;
use sqlx::Row;
use std::fs;
use std::path::Path;
use tokio::runtime::Runtime;

// Check if a database file exists, and create one if it does not.
pub fn app_database_init() {
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        if !db_file_exists() {
            create_db_file().await;
            create_tables().await;
            seed_database().await;
        }
    });
}

// Create the database
async fn create_db_file() {
    let db_path = get_db_path();
    let db_dir = Path::new(&db_path).parent().unwrap();

    // If the parent directory does not exist, create it.
    if !db_dir.exists() {
        match fs::create_dir_all(db_dir) {
            Ok(_) => println!("Created database directory at {:?}", db_dir),
            Err(e) => {
                eprintln!("Failed to create database directory: {}", e);
                return;
            }
        }
    }

    // Create the database file and establish a connection to initialize it
    match SqlitePool::connect(&format!("sqlite://{}", db_path)).await {
        Ok(pool) => {
            println!("Successfully created database at {}", db_path);
            pool.close().await;
        }
        Err(e) => {
            eprintln!("Failed to create database: {}", e);
            // Try to create the file first
            if let Err(e) = fs::File::create(&db_path) {
                eprintln!("Failed to create database file: {}", e);
            }
        }
    }
}

// Check whether the database file exists.
fn db_file_exists() -> bool {
    let db_path = get_db_path();
    Path::new(&db_path).exists()
}

// Get the path where the database file should be located.
pub fn get_db_path() -> String {
    let home_dir = dirs::home_dir().unwrap();
    let db_path = home_dir
        .join(".config")
        .join("database-helper-application")
        .join("local.db");
    db_path.to_str().unwrap().to_string()
}

async fn create_tables() {
    let database_url = get_db_path();
    let pool = SqlitePool::connect(&format!("sqlite://{}", database_url))
        .await
        .unwrap();

    // user_authentication_type
    sqlx::query(
        "
        CREATE TABLE IF NOT EXISTS user_authentication_type (
            id INTEGER PRIMARY KEY AUTOINCREMENT UNIQUE,
            type TEXT NOT NULL,
            description TEXT NOT NULL
        );
        ",
    )
    .execute(&pool)
    .await
    .unwrap();

    // user
    sqlx::query(
        "
        CREATE TABLE IF NOT EXISTS user (
            id INTEGER PRIMARY KEY AUTOINCREMENT UNIQUE,
            authentication_type_id INTEGER NOT NULL,
            name TEXT NOT NULL,
            password TEXT NOT NULL,
            email TEXT NOT NULL UNIQUE,
            secondary_email TEXT,
            FOREIGN KEY(authentication_type_id) REFERENCES user_authentication_type(id)
        );
        ",
    )
    .execute(&pool)
    .await
    .unwrap();

    // datasource
    sqlx::query(
        "
        CREATE TABLE IF NOT EXISTS datasource (
            id INTEGER PRIMARY KEY AUTOINCREMENT UNIQUE,
            type TEXT NOT NULL UNIQUE,
            description TEXT NOT NULL
        );
        ",
    )
    .execute(&pool)
    .await
    .unwrap();

    // database_connection
    sqlx::query(
        "
        CREATE TABLE IF NOT EXISTS database_connection (
            id INTEGER PRIMARY KEY AUTOINCREMENT UNIQUE,
            user_id INTEGER,
            datasource_id INTEGER,
            connection_name TEXT NOT NULL,
            url TEXT NOT NULL,
            FOREIGN KEY(user_id) REFERENCES user(id),
            FOREIGN KEY(datasource_id) REFERENCES datasource(id)
        );
        ",
    )
    .execute(&pool)
    .await
    .unwrap();

    // content_type
    sqlx::query(
        "
        CREATE TABLE IF NOT EXISTS content_type (
            id INTEGER PRIMARY KEY AUTOINCREMENT UNIQUE,
            name TEXT NOT NULL UNIQUE,
            description TEXT NOT NULL
        );
        ",
    )
    .execute(&pool)
    .await
    .unwrap();

    // query_file
    sqlx::query(
        "
        CREATE TABLE IF NOT EXISTS query_file (
            id INTEGER PRIMARY KEY AUTOINCREMENT UNIQUE,
            name TEXT NOT NULL,
            description TEXT NOT NULL
        );
        ",
    )
    .execute(&pool)
    .await
    .unwrap();

    // query_block
    sqlx::query(
        "
        CREATE TABLE IF NOT EXISTS query_block (
            id INTEGER PRIMARY KEY AUTOINCREMENT UNIQUE,
            query_file_id INTEGER NOT NULL,
            content_type_id INTEGER NOT NULL,
            serial_order INTEGER,
            query_content_block TEXT,
            FOREIGN KEY(query_file_id) REFERENCES query_file(id),
            FOREIGN KEY(content_type_id) REFERENCES content_type(id)
        );
        ",
    )
    .execute(&pool)
    .await
    .unwrap();

    // database_file_collection
    sqlx::query(
        "
        CREATE TABLE IF NOT EXISTS database_file_collection (
            id INTEGER PRIMARY KEY AUTOINCREMENT UNIQUE,
            database_connection_id INTEGER NOT NULL,
            query_file_id INTEGER NOT NULL,
            FOREIGN KEY(database_connection_id) REFERENCES database_connection(id),
            FOREIGN KEY(query_file_id) REFERENCES query_file(id)
        );
        ",
    )
    .execute(&pool)
    .await
    .unwrap();
}

async fn seed_database() {
    let database_url = get_db_path();
    let pool = SqlitePool::connect(&format!("sqlite://{}", database_url))
        .await
        .unwrap();

    let datasource_inserts = vec![
        ("PostgreSQL", "A dependable open-source relational database celebrated for its sophisticated capabilities."),
        ("SQLite", "A streamlined, self-contained relational database perfect for local storage and embedded uses."),
        ("MongoDB", "An adaptable NoSQL document database built for high-throughput and horizontally scalable applications."),
        ("MySQL", "A broadly used open-source system for managing relational databases."),
        ("MSSQL", "Microsoft SQL Server, a widely implemented relational database platform."),
        ("MariaDB", "An open-source relational database system driven by community innovation."),
    ];

    for (db_type, db_desc) in datasource_inserts {
        sqlx::query("INSERT INTO datasource (type, description) VALUES (?, ?)")
            .bind(db_type)
            .bind(db_desc)
            .execute(&pool)
            .await
            .unwrap();
    }

    let user_auth_inserts = vec![
        ("guest", "Empty authentication type, your workspaces will be public and can be access by other users on this device."),
        ("email and password", "Basic authentication type, it will grant you your own private workspace."),
        ("Verified", "Advanced authentication type, it will allow you to collaborate with others."),
    ];

    for (auth_type, auth_desc) in user_auth_inserts {
        sqlx::query("INSERT INTO user_authentication_type (type, description) VALUES (?, ?)")
            .bind(auth_type)
            .bind(auth_desc)
            .execute(&pool)
            .await
            .unwrap();
    }

    let content_type_inserts = vec![
        ("MD", "A Markdown content type."),
        ("JSON", "A JSON content type."),
        ("SQL", "An SQL content type."),
    ];

    for (content_type, content_desc) in content_type_inserts {
        sqlx::query("INSERT INTO content_type (name, description) VALUES (?, ?)")
            .bind(content_type)
            .bind(content_desc)
            .execute(&pool)
            .await
            .unwrap();
    }
}

#[tauri::command]
pub async fn get_user_by_email(email: String, password: String) -> Result<User, String> {
    let database_url = get_db_path();
    let pool = SqlitePool::connect(&format!("sqlite://{}", database_url))
        .await
        .map_err(|e| e.to_string())?;

    let user = sqlx::query(
        "SELECT id, authentication_type_id, name, password, email, secondary_email FROM user WHERE email = ? AND password = ?"
    )
    .bind(email)
    .bind(password)
    .fetch_optional(&pool)
    .await
    .map_err(|e| e.to_string())?;

    match user {
        Some(row) => Ok(User {
            id: row.get(0),
            authentication_type_id: row.get(1),
            name: row.get(2),
            password: row.get(3),
            email: row.get(4),
            secondary_email: row.get(5),
        }),
        None => Err("User not found".to_string()),
    }
}

#[tauri::command]
pub async fn signup_user(name: String, email: String, password: String) -> Result<(), String> {
    let database_url = get_db_path();
    let pool = SqlitePool::connect(&format!("sqlite://{}", database_url))
        .await
        .map_err(|e| e.to_string())?;

    sqlx::query(
        "INSERT INTO user (name, email, password, authentication_type_id) VALUES (?, ?, ?, ?)",
    )
    .bind(name)
    .bind(email)
    .bind(password)
    .bind(1) // authentication_type_id for email and password
    .execute(&pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub async fn get_datasource() -> Result<Vec<DatasourceStruct>, String> {
    let database_url = get_db_path();
    let pool = SqlitePool::connect(&format!("sqlite://{}", database_url))
        .await
        .map_err(|e| e.to_string())?;

    let rows = sqlx::query("SELECT id, type, description FROM datasource")
        .fetch_all(&pool)
        .await
        .map_err(|e| e.to_string())?;

    let mut datasources = Vec::new();
    for row in rows {
        datasources.push(DatasourceStruct {
            id: row.get(0),
            name: row.get(1),
            description: row.get(2),
        });
    }

    Ok(datasources)
}

#[tauri::command]
pub async fn get_content_type() -> Result<Vec<ContentTypeStruct>, String> {
    let database_url = get_db_path();
    let pool = SqlitePool::connect(&format!("sqlite://{}", database_url))
        .await
        .map_err(|e| e.to_string())?;

    let rows = sqlx::query("SELECT id, name, description FROM content_type")
        .fetch_all(&pool)
        .await
        .map_err(|e| e.to_string())?;

    let mut content_types = Vec::new();
    for row in rows {
        content_types.push(ContentTypeStruct {
            id: row.get(0),
            name: row.get(1),
            description: row.get(2),
        });
    }

    Ok(content_types)
}

#[tauri::command]
pub async fn get_database_connection() -> Result<Vec<DatabaseConnectionStruct>, String> {
    let database_url = get_db_path();
    let pool = SqlitePool::connect(&format!("sqlite://{}", database_url))
        .await
        .map_err(|e| e.to_string())?;

    let rows = sqlx::query(
        "SELECT id, user_id, datasource_id, connection_name, url FROM database_connection",
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| e.to_string())?;

    let mut connections = Vec::new();
    for row in rows {
        connections.push(DatabaseConnectionStruct {
            id: row.get(0),
            user_id: row.get(1),
            datasource_id: row.get(2),
            connection_name: row.get(3),
            url: row.get(4),
        });
    }

    Ok(connections)
}

#[tauri::command]
pub async fn create_file_for_database(name: String, description: String) -> Result<i64, String> {
    print!("create_file_for_database , {} , {}", name, description);
    let database_url = get_db_path();
    let pool = SqlitePool::connect(&format!("sqlite://{}", database_url))
        .await
        .map_err(|e| e.to_string())?;

    let result =
        sqlx::query("INSERT INTO query_file (name, description) VALUES (?, ?) RETURNING id")
            .bind(name)
            .bind(description)
            .fetch_one(&pool)
            .await
            .map_err(|e| e.to_string())?;

    let id: i64 = result.get("id");

    Ok(id)
}

#[tauri::command]
pub async fn store_file_with_database(
    database_connection_id: i64,
    query_file_id: i64,
) -> Result<(), String> {
    let database_url = get_db_path();
    let pool = SqlitePool::connect(&format!("sqlite://{}", database_url))
        .await
        .map_err(|e| e.to_string())?;

    sqlx::query("INSERT INTO database_file_collection (database_connection_id, query_file_id) VALUES (?, ?)")
        .bind(database_connection_id)
        .bind(query_file_id)
        .execute(&pool)
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub async fn get_file_collection(
    database_connection_id: i64,
) -> Result<Vec<QueryFileStruct>, String> {
    let database_url = get_db_path();
    let pool = SqlitePool::connect(&format!("sqlite://{}", database_url))
        .await
        .map_err(|e| e.to_string())?;

    let rows = sqlx::query(
        "SELECT id, name, description FROM query_file WHERE id IN (SELECT query_file_id FROM database_file_collection WHERE database_connection_id = ?)",
    )
    .bind(database_connection_id)
    .fetch_all(&pool)
    .await
    .map_err(|e| e.to_string())?;

    let mut files = Vec::new();
    for row in rows {
        files.push(QueryFileStruct {
            id: row.get(0),
            name: row.get(1),
            description: row.get(2),
        });
    }

    Ok(files)
}

#[tauri::command]
pub async fn create_new_query_block(
    query_file_id: i64,
    content_type_id: i64,
    serial_order: i64,
    query_content_block: String,
) -> Result<i64, String> {
    let database_url = get_db_path();
    let pool = SqlitePool::connect(&format!("sqlite://{}", database_url))
        .await
        .map_err(|e| e.to_string())?;

    let result = sqlx::query("INSERT INTO query_block (query_file_id, content_type_id, serial_order, query_content_block) VALUES (?, ?, ?, ?) RETURNING id")
        .bind(query_file_id)
        .bind(content_type_id)
        .bind(serial_order)
        .bind(query_content_block)
        .fetch_one(&pool)
        .await
        .map_err(|e| e.to_string())?;

    let id: i64 = result.get("id");

    Ok(id)
}

#[tauri::command]
pub async fn get_query_blocks(query_file_id: i64) -> Result<Vec<QueryBlockStruct>, String> {
    let database_url = get_db_path();
    let pool = SqlitePool::connect(&format!("sqlite://{}", database_url))
        .await
        .map_err(|e| e.to_string())?;

    let rows = sqlx::query(
        "SELECT id, query_file_id, content_type_id, serial_order, query_content_block FROM query_block WHERE query_file_id = ?",
    )
    .bind(query_file_id)
    .fetch_all(&pool)
    .await
    .map_err(|e| e.to_string())?;

    let mut blocks = Vec::new();
    for row in rows {
        blocks.push(QueryBlockStruct {
            id: row.get(0),
            query_file_id: row.get(1),
            content_type_id: row.get(2),
            serial_order: row.get(3),
            query_content_block: row.get(4),
        });
    }

    Ok(blocks)
}

#[tauri::command]
pub async fn save_query_content_to_the_block(
    query_block_id: i64,
    query_content_block: String,
) -> Result<(), String> {
    let database_url = get_db_path();
    let pool = SqlitePool::connect(&format!("sqlite://{}", database_url))
        .await
        .map_err(|e| e.to_string())?;

    sqlx::query("UPDATE query_block SET query_content_block = ? WHERE id = ?")
        .bind(query_content_block)
        .bind(query_block_id)
        .execute(&pool)
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub async fn update_query_block_content_type_id(
    query_block_id: i64,
    content_type_id: i64,
) -> Result<(), String> {
    let database_url = get_db_path();
    let pool = SqlitePool::connect(&format!("sqlite://{}", database_url))
        .await
        .map_err(|e| e.to_string())?;

    sqlx::query("UPDATE query_block SET content_type_id = ? WHERE id = ?")
        .bind(content_type_id)
        .bind(query_block_id)
        .execute(&pool)
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}
