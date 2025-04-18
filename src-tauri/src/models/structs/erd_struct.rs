use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ColumnInfo {
    pub name: String,
    pub data_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TableInfo {
    pub name: String,
    pub columns: Vec<ColumnInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CollectionInfo {
    pub database_name: String,
    pub collection_name: String,
}
