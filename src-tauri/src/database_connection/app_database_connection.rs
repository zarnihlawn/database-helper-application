use crate::models::structs::schema_struct::{
    ContentType, Datasource, DatasourceAuthenticationType, User,
};

use sqlite::{open, State};
use std::fs;
use std::path::Path;

// Check if a database file exists, and create one if it does not.
pub fn app_database_init() {
    if !db_file_exists() {
        create_db_file();
        create_tables();
        seed_database();
    }
}

// Create the database file.
fn create_db_file() {
    let db_path = get_db_path();
    let db_dir = Path::new(&db_path).parent().unwrap();

    // If the parent directory does not exist, create it.
    if !db_dir.exists() {
        fs::create_dir_all(db_dir).unwrap();
    }

    // Create the database file.
    fs::File::create(db_path).unwrap();
}

// Check whether the database file exists.
fn db_file_exists() -> bool {
    let db_path = get_db_path();
    Path::new(&db_path).exists()
}

// Get the path where the database file should be located.
pub fn get_db_path() -> String {
    let home_dir = dirs::home_dir().unwrap();
    home_dir.to_str().unwrap().to_string() + "/.config/database-helper-application/local.db"
}

fn create_tables() {
    let database_url = get_db_path();
    let connection = open(database_url).unwrap();

    // user_authentication_type
    connection
        .execute(
            "
            CREATE TABLE IF NOT EXISTS user_authentication_type (
                id INTEGER PRIMARY KEY AUTOINCREMENT UNIQUE,
                type TEXT NOT NULL,
                description TEXT NOT NULL
            );
            ",
        )
        .unwrap();

    // user
    connection
        .execute(
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
        .unwrap();

    // user_previous_password
    connection
        .execute(
            "
            CREATE TABLE IF NOT EXISTS user_previous_password (
                id INTEGER PRIMARY KEY AUTOINCREMENT UNIQUE,
                user_id INTEGER NOT NULL,
                password TEXT NOT NULL,
                FOREIGN KEY(user_id) REFERENCES user(id)
            );
            ",
        )
        .unwrap();

    // datasource
    connection
        .execute(
            "
            CREATE TABLE IF NOT EXISTS datasource (
                id INTEGER PRIMARY KEY AUTOINCREMENT UNIQUE,
                type TEXT NOT NULL UNIQUE,
                description TEXT NOT NULL
            );
            ",
        )
        .unwrap();

    // database_connection
    connection
        .execute(
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
        .unwrap();

    // content_type
    connection
        .execute(
            "
            CREATE TABLE IF NOT EXISTS content_type (
                id INTEGER PRIMARY KEY AUTOINCREMENT UNIQUE,
                name TEXT NOT NULL UNIQUE,
                description TEXT NOT NULL
            );
            ",
        )
        .unwrap();

    // query_file
    connection
        .execute(
            "
            CREATE TABLE IF NOT EXISTS query_file (
                id INTEGER PRIMARY KEY AUTOINCREMENT UNIQUE,
                name TEXT NOT NULL,
                description TEXT NOT NULL
            );
            ",
        )
        .unwrap();

    // query_block
    connection
        .execute(
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
        .unwrap();

    // database_file_collection
    connection
        .execute(
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
        .unwrap();
}

fn seed_database() {
    let database_url = get_db_path();
    let connection = open(database_url).unwrap();

    let datasource_inserts = vec![
        ("PostgreSQL", "A powerful, open-source relational database known for its reliability and advanced features."),
        ("SQLite", "A lightweight, self-contained relational database ideal for embedded systems and local storage."),
        ("MongoDB", "A flexible NoSQL document database designed for scalable and high-performance applications."),
        ("Oracle", "A robust enterprise-grade relational database optimized for complex data management and high availability."),
        ("MySQL", "A widely used relational database managing data in structured tables for diverse applications."),
    ];

    for (db_type, db_desc) in datasource_inserts {
        let mut statement = connection
            .prepare("INSERT INTO datasource (type, description) VALUES (?, ?);")
            .unwrap();
        statement.bind((1, db_type)).unwrap();
        statement.bind((2, db_desc)).unwrap();
        statement.next().unwrap();
    }

    let user_auth_inserts = vec![
        ("guest", "Empty authentication type, your workspaces will be public and can be access by other users on this device."),
        ("email and password", "Basic authentication type, it will grant you your own private workspace."),
        ("Verified", "Advanced authentication type, it will allow you to collaborate with other."),
    ];

    for (auth_type, auth_desc) in user_auth_inserts {
        let mut statement = connection
            .prepare("INSERT INTO user_authentication_type (type, description) VALUES (?, ?);")
            .unwrap();
        statement.bind((1, auth_type)).unwrap();
        statement.bind((2, auth_desc)).unwrap();
        statement.next().unwrap();
    }
}

#[tauri::command]
pub async fn get_user_by_email(email: String, password: String) -> Result<User, String> {
    let database_url = get_db_path();
    let connection = open(database_url).map_err(|e| e.to_string())?;

    let query = "SELECT * FROM user WHERE email = ? AND password = ?";
    let mut statement = connection.prepare(query).map_err(|e| e.to_string())?;

    statement
        .bind((1, email.as_str()))
        .map_err(|e| e.to_string())?;
    statement
        .bind((2, password.as_str()))
        .map_err(|e| e.to_string())?;

    match statement.next() {
        Ok(State::Row) => {
            let id = statement.read::<i64, _>(0).map_err(|e| e.to_string())? as i32;
            let authentication_type_id =
                statement.read::<i64, _>(1).map_err(|e| e.to_string())? as i32;
            let name = statement.read::<String, _>(2).map_err(|e| e.to_string())?;
            let password = statement.read::<String, _>(3).map_err(|e| e.to_string())?;
            let email = statement.read::<String, _>(4).map_err(|e| e.to_string())?;
            let secondary_email = statement
                .read::<Option<String>, _>(5)
                .map_err(|e| e.to_string())?;

            Ok(User {
                id,
                authentication_type_id,
                name,
                password,
                email,
                secondary_email,
            })
        }
        _ => Err("User not found".to_string()),
    }
}

#[tauri::command]
pub async fn signup_user(name: String, email: String, password: String) -> Result<(), String> {
    let database_url = get_db_path();
    let connection = open(database_url).map_err(|e| e.to_string())?;

    // Fixed SQL query (INSERT instead of INERT and proper parameters)
    let query =
        "INSERT INTO user (name, email, password, authentication_type_id) VALUES (?, ?, ?, ?)";

    let mut statement = connection.prepare(query).map_err(|e| e.to_string())?;

    // Binding parameters (index starts at 1)
    statement
        .bind((1, name.as_str()))
        .map_err(|e| e.to_string())?;
    statement
        .bind((2, email.as_str()))
        .map_err(|e| e.to_string())?;
    statement
        .bind((3, password.as_str()))
        .map_err(|e| e.to_string())?;
    statement.bind((4, 1)).map_err(|e| e.to_string())?;

    // Execute the statement
    if let State::Done = statement.next().map_err(|e| e.to_string())? {
        Ok(())
    } else {
        Err("Failed to execute statement".to_string())
    }
}

#[tauri::command]
pub async fn get_datasource() -> Result<Vec<Datasource>, String> {
    let database_url = get_db_path();
    let connection = open(database_url).map_err(|e| e.to_string())?;

    let query = "SELECT * FROM datasource";
    let mut statement = connection.prepare(query).map_err(|e| e.to_string())?;

    let mut datasources = Vec::new();

    while let Ok(State::Row) = statement.next() {
        let id = statement.read::<i64, _>(0).map_err(|e| e.to_string())? as i32;
        let r#type = statement.read::<String, _>(1).map_err(|e| e.to_string())?;
        let description = statement.read::<String, _>(2).map_err(|e| e.to_string())?;

        datasources.push(Datasource {
            id,
            r#type,
            description,
        });
    }

    Ok(datasources)
}

#[tauri::command]
pub async fn get_content_type() -> Result<Vec<ContentType>, String> {
    let database_url = get_db_path();
    let connection = open(database_url).map_err(|e| e.to_string())?;

    let query = "SELECT * FROM content_type";
    let mut statement = connection.prepare(query).map_err(|e| e.to_string())?;

    let mut content_types = Vec::new();

    while let Ok(State::Row) = statement.next() {
        let id = statement.read::<i64, _>(0).map_err(|e| e.to_string())? as i32;
        let name = statement.read::<String, _>(1).map_err(|e| e.to_string())?;
        let description = statement.read::<String, _>(2).map_err(|e| e.to_string())?;

        content_types.push(ContentType {
            id,
            name,
            description,
        });
    }

    Ok(content_types)
}

#[tauri::command]
pub async fn get_datasource_authentication_type(
) -> Result<Vec<DatasourceAuthenticationType>, String> {
    let database_url = get_db_path();
    let connection = open(database_url).map_err(|e| e.to_string())?;

    let query = "SELECT * FROM datasource_authentication_type";
    let mut statement = connection.prepare(query).map_err(|e| e.to_string())?;

    let mut datasource_authentication_types = Vec::new();

    while let Ok(State::Row) = statement.next() {
        let id = statement.read::<i64, _>(0).map_err(|e| e.to_string())? as i32;
        let r#type = statement.read::<String, _>(1).map_err(|e| e.to_string())?;
        let description = statement.read::<String, _>(2).map_err(|e| e.to_string())?;

        datasource_authentication_types.push(DatasourceAuthenticationType {
            id,
            r#type,
            description,
        });
    }

    Ok(datasource_authentication_types)
}
