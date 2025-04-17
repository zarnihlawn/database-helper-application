// ContentType
#[derive(serde::Serialize)]
pub struct ContentTypeStruct {
    pub id: i32,
    pub name: String,
    pub description: String,
}

pub struct ContentTypeDtoStruct {
    pub name: String,
    pub description: String,
}

// DatabaseConnection
pub struct DatabaseConnectionStruct {
    pub id: i32,
    pub user_id: Option<i32>,
    pub datasource_id: i32,
    pub connection_name: String,
    pub url: String,
}

pub struct DatabaseConnectionDtoStruct {
    pub user_id: Option<i32>,
    pub datasource_id: i32,
    pub connection_name: String,
    pub url: String,
}

// DatabaseFileCollection
pub struct DatabaseFileCollectionStruct {
    pub id: i32,
    pub database_connection_id: i32,
    pub query_file_id: i32,
}

pub struct DatabaseFileCollectionDtoStruct {
    pub database_connection_id: i32,
    pub query_file_id: i32,
}

// Datasource
#[derive(serde::Serialize)]
pub struct DatasourceStruct {
    pub id: i32,
    pub name: String,
    pub description: String,
}

pub struct DatasourceDto {
    pub name: String,
    pub description: String,
}

// QueryBlock
pub struct QueryBlockStruct {
    pub id: i32,
    pub query_file_id: i32,
    pub content_type_id: i32,
    pub serial_order: i32,
    pub query_content_block: String,
}

pub struct QueryBlockDtoStruct {
    pub query_file_id: i32,
    pub content_type_id: i32,
    pub serial_order: i32,
    pub query_content_block: String,
}

// QueryFile
pub struct QueryFileStruct {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
}

pub struct QueryFileDtoStruct {
    pub name: String,
    pub description: Option<String>,
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
