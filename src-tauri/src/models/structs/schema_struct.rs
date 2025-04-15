// ContentType
#[derive(serde::Serialize)]
pub struct ContentType {
    pub id: i32,
    pub name: String,
    pub description: String,
}

pub struct ContentTypeDto {
    pub name: String,
    pub description: String,
}

// Database
pub struct Database {
    pub id: i32,
    pub datasource_connection_id: i32,
    pub user_id: i32,
}

pub struct DatabaseDto {
    pub datasource_connection_id: i32,
    pub user_id: i32,
}

// DatasourceConnection
pub struct DatasourceConnection {
    pub id: i32,
    pub datasource_id: i32,
    pub datasource_authentication_type_id: Option<i32>,
    pub connection_name: Option<String>,
    pub host: Option<String>,
    pub port: Option<i32>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub driver: Option<String>,
    pub sid: Option<String>,
    pub url: Option<String>,
    pub path: Option<String>,
}

pub struct DatasourceConnectionDto {
    pub datasource_id: i32,
    pub datasource_authentication_type_id: Option<i32>,
    pub connection_name: Option<String>,
    pub host: Option<String>,
    pub port: Option<i32>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub driver: Option<String>,
    pub sid: Option<String>,
    pub url: Option<String>,
    pub path: Option<String>,
}

// Datasource
#[derive(serde::Serialize)]
pub struct Datasource {
    pub id: i32,
    pub r#type: String, // Use r# to escape Rust's keyword 'type'
    pub description: String,
}

pub struct DatasourceDto {
    pub r#type: String, // Use r# to escape Rust's keyword 'type'
    pub description: String,
}

// DatasourceAuthenticationType
#[derive(serde::Serialize)]
pub struct DatasourceAuthenticationType {
    pub id: i32,
    pub r#type: String, // Use r# to escape Rust's keyword 'type'
    pub description: String,
}

pub struct DatasourceAuthenticationTypeDto {
    pub r#type: String, // Use r# to escape Rust's keyword 'type'
    pub description: String,
}

// QueryBlock
pub struct QueryBlock {
    pub id: i32,
    pub query_file_id: i32,
    pub content_type_id: i32,
    pub serial_order: i32,
    pub query_content_block: String,
}

pub struct QueryBlockDto {
    pub query_file_id: i32,
    pub content_type_id: i32,
    pub serial_order: i32,
    pub query_content_block: String,
}

// QueryFile
pub struct QueryFile {
    pub id: i32,
    pub name: String,
    pub content: String,
}

pub struct QueryFileDto {
    pub name: String,
    pub description: String,
}

// User
#[derive(serde::Serialize)]
pub struct User {
    pub id: i32,
    pub authentication_type_id: i32,
    pub name: String,
    pub password: String,
    pub email: String,
    pub secondary_email: Option<String>,
}

pub struct UserDto {
    pub authorization_level_id: i32,
    pub authentication_type_id: i32,
    pub name: String,
    pub password: String,
    pub email: String,
    pub secondary_email: Option<String>,
}

// UserCookie
pub struct UserCookie {
    pub id: i32,
    pub name: String,
    pub email: String,
}

// UserAuthenticationType
pub struct UserAuthenticationType {
    pub id: i32,
    pub name: String,
    pub description: String,
}

pub struct UserAuthenticationTypeDto {
    pub name: String,
    pub description: String,
}

// UserAuthorizationLevel
pub struct UserAuthorizationLevel {
    pub id: i32,
    pub level: i32,
    pub description: String,
}

pub struct UserAuthorizationLevelDto {
    pub level: i32,
    pub description: String,
}

// UserPreviousPassword
pub struct UserPreviousPassword {
    pub id: i32,
    pub user_id: i32,
    pub password: String,
}

// userprevioupassword dto, I corrected the repeated interface.
pub struct UserPreviousPasswordDto {
    pub user_id: i32,
    pub password: String,
}
