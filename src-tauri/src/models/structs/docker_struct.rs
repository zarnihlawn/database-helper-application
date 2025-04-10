use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize)]
pub struct DockerImage {
    pub repository: String,
    pub tag: String,
    pub image_id: String,
    pub created: String,
    pub size: String,
    pub labels: HashMap<String, String>,
}

#[derive(Debug, Serialize)]
pub struct DockerContainer {
    pub id: String,
    pub name: String,
    pub image: String,
    pub status: String,
    pub state: String,
    pub created: String,
    pub ports: Vec<String>,
    pub labels: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DockerHubImage {
    pub name: String,
    pub description: String,
    pub star_count: i32,
    pub pull_count: i32,
    pub official: bool,
}
