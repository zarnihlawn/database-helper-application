use dotenv::dotenv;
use sqlite::{open, State};
use std::env::var;

use crate::models::structs::schema_struct::{Datasource, User};

#[tauri::command]
pub async fn get_user_by_email(email: String, password: String) -> Result<User, String> {
    dotenv().ok();

    let database_url = var("DATABASE_URL").map_err(|e| e.to_string())?;
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
            let authorization_level_id =
                statement.read::<i64, _>(1).map_err(|e| e.to_string())? as i32;
            let authentication_type_id =
                statement.read::<i64, _>(2).map_err(|e| e.to_string())? as i32;
            let name = statement.read::<String, _>(3).map_err(|e| e.to_string())?;
            let password = statement.read::<String, _>(4).map_err(|e| e.to_string())?;
            let email = statement.read::<String, _>(5).map_err(|e| e.to_string())?;
            let secondary_email = statement
                .read::<Option<String>, _>(6)
                .map_err(|e| e.to_string())?;

            Ok(User {
                id,
                authorization_level_id,
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
    dotenv().ok();

    let database_url = var("DATABASE_URL").map_err(|e| e.to_string())?;
    let connection = open(database_url).map_err(|e| e.to_string())?;

    // Fixed SQL query (INSERT instead of INERT and proper parameters)
    let query = "INSERT INTO user (name, email, password, authorization_level_id, authentication_type_id) VALUES (?, ?, ?, ?, ?)";

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
    statement.bind((5, 1)).map_err(|e| e.to_string())?;

    // Execute the statement
    if let State::Done = statement.next().map_err(|e| e.to_string())? {
        Ok(())
    } else {
        Err("Failed to execute statement".to_string())
    }
}

#[tauri::command]
pub async fn get_datasource() -> Result<Vec<Datasource>, String> {
    dotenv().ok();

    let database_url = var("DATABASE_URL").map_err(|e| e.to_string())?;
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

