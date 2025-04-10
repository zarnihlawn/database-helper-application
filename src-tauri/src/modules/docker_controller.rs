use bollard::container::ListContainersOptions;
use bollard::image::ListImagesOptions;
use bollard::models::ImageSummary;
use bollard::Docker;
use std::default::Default;
use std::process::Command;

use crate::models::structs::docker_struct::{DockerContainer, DockerHubImage, DockerImage};

#[tauri::command]
pub async fn check_docker_status() -> Result<(bool, String), String> {
    let docker = Docker::connect_with_local_defaults()
        .map_err(|e| format!("Failed to connect to Docker: {}", e))?;

    match docker.ping().await {
        Ok(_) => Ok((true, "Docker is running".to_string())),
        Err(e) => Ok((false, format!("Docker connection error: {}", e))),
    }
}

#[tauri::command]
pub async fn start_docker_service() -> Result<bool, String> {
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(&["/C", "docker desktop start"])
            .output()
    } else if cfg!(target_os = "linux") {
        Command::new("systemctl")
            .args(&["start", "docker"])
            .output()
    } else if cfg!(target_os = "macos") {
        Command::new("open")
            .args(&["/Applications/Docker.app"])
            .output()
    } else {
        return Err("Unsupported operating system".to_string());
    };

    match output {
        Ok(output) => {
            if output.status.success() {
                Ok(true)
            } else {
                Err(String::from_utf8_lossy(&output.stderr).to_string())
            }
        }
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub async fn stop_docker_service() -> Result<bool, String> {
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(&["/C", "docker desktop stop"])
            .output()
    } else if cfg!(target_os = "linux") {
        Command::new("systemctl").args(&["stop", "docker"]).output()
    } else if cfg!(target_os = "macos") {
        Command::new("osascript")
            .args(&["-e", "quit app \\\"Docker\\\""])
            .output()
    } else {
        return Err("Unsupported operating system".to_string());
    };

    match output {
        Ok(output) => {
            if output.status.success() {
                Ok(true)
            } else {
                Err(String::from_utf8_lossy(&output.stderr).to_string())
            }
        }
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub async fn restart_docker_service() -> Result<bool, String> {
    let stop_result = stop_docker_service().await;
    match stop_result {
        Ok(_) => start_docker_service().await,
        Err(e) => Err(e),
    }
}

#[tauri::command]
pub async fn get_all_docker_images() -> Result<Vec<DockerImage>, String> {
    let docker = Docker::connect_with_local_defaults()
        .map_err(|e| format!("Failed to connect to Docker: {}", e))?;

    let images = docker
        .list_images(Some(ListImagesOptions::<String> {
            all: true,
            ..Default::default()
        }))
        .await
        .map_err(|e| format!("Failed to list Docker images: {}", e))?;

    let docker_images: Vec<DockerImage> = images
        .into_iter()
        .map(|image: ImageSummary| {
            let repo_tags = image.repo_tags;
            let (repository, tag) = if let Some(first_tag) = repo_tags.first() {
                let parts: Vec<&str> = first_tag.split(':').collect();
                if parts.len() == 2 {
                    (parts[0].to_string(), parts[1].to_string())
                } else {
                    (first_tag.to_string(), "latest".to_string())
                }
            } else {
                ("<none>".to_string(), "<none>".to_string())
            };

            DockerImage {
                repository,
                tag,
                image_id: image.id,
                created: image.created.to_string(),
                size: format!("{} MB", image.size as f64 / 1_048_576.0),
                labels: image.labels,
            }
        })
        .collect();

    Ok(docker_images)
}

#[tauri::command]
pub async fn get_all_docker_containers() -> Result<Vec<DockerContainer>, String> {
    let docker = Docker::connect_with_local_defaults()
        .map_err(|e| format!("Failed to connect to Docker: {}", e))?;

    let containers = docker
        .list_containers(Some(ListContainersOptions::<String> {
            all: true,
            ..Default::default()
        }))
        .await
        .map_err(|e| format!("Failed to list Docker containers: {}", e))?;

    let docker_containers: Vec<DockerContainer> = containers
        .into_iter()
        .map(|container| {
            let name = container
                .names
                .unwrap_or_default()
                .first()
                .map(|n| n.trim_start_matches('/').to_string())
                .unwrap_or_default();

            let ports = container
                .ports
                .unwrap_or_default()
                .into_iter()
                .map(|port| {
                    format!(
                        "{}:{}/{}",
                        port.public_port.unwrap_or_default(),
                        port.private_port,
                        port.typ
                            .map(|t| t.to_string())
                            .unwrap_or_else(|| "tcp".to_string())
                    )
                })
                .collect();

            DockerContainer {
                id: container.id.unwrap_or_default(),
                name,
                image: container.image.unwrap_or_default(),
                status: container.status.unwrap_or_default(),
                state: container.state.unwrap_or_default(),
                created: container.created.unwrap_or_default().to_string(),
                ports,
                labels: container.labels.unwrap_or_default(),
            }
        })
        .collect();

    Ok(docker_containers)
}

#[tauri::command]
pub async fn start_docker_container(container_id: String) -> Result<bool, String> {
    let docker = Docker::connect_with_local_defaults()
        .map_err(|e| format!("Failed to connect to Docker: {}", e))?;

    docker
        .start_container::<String>(&container_id, None)
        .await
        .map_err(|e| format!("Failed to start container: {}", e))?;

    Ok(true)
}

#[tauri::command]
pub async fn stop_docker_container(container_id: String) -> Result<bool, String> {
    let docker = Docker::connect_with_local_defaults()
        .map_err(|e| format!("Failed to connect to Docker: {}", e))?;

    docker
        .stop_container(&container_id, None)
        .await
        .map_err(|e| format!("Failed to stop container: {}", e))?;

    Ok(true)
}

#[tauri::command]
pub async fn delete_docker_container(container_id: String) -> Result<bool, String> {
    let docker = Docker::connect_with_local_defaults()
        .map_err(|e| format!("Failed to connect to Docker: {}", e))?;

    docker
        .remove_container(&container_id, None)
        .await
        .map_err(|e| format!("Failed to delete container: {}", e))?;

    Ok(true)
}

#[tauri::command]
pub async fn delete_docker_image(image_id: String) -> Result<bool, String> {
    let docker = Docker::connect_with_local_defaults()
        .map_err(|e| format!("Failed to connect to Docker: {}", e))?;

    docker
        .remove_image(&image_id, None, None)
        .await
        .map_err(|e| format!("Failed to delete image: {}", e))?;

    Ok(true)
}

#[tauri::command]
pub async fn search_docker_image(query: String) -> Result<Vec<DockerHubImage>, String> {
    let client = reqwest::Client::new();
    let url = format!(
        "https://hub.docker.com/v2/search/repositories?query={}&page=1&page_size=100",
        query
    );

    let response = client
        .get(&url)
        .send()
        .await
        .map_err(|e| format!("Failed to search Docker Hub: {}", e))?;

    let search_result: serde_json::Value = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse Docker Hub response: {}", e))?;

    let results = search_result
        .get("results")
        .and_then(|r| r.as_array())
        .ok_or("Invalid response format from Docker Hub")?;

    let images: Vec<DockerHubImage> = results
        .iter()
        .filter_map(|result| {
            Some(DockerHubImage {
                name: result.get("repo_name")?.as_str()?.to_string(),
                description: result.get("short_description")?.as_str()?.to_string(),
                star_count: result.get("star_count")?.as_i64()? as i32,
                pull_count: result.get("pull_count")?.as_i64()? as i32,
                official: result.get("is_official")?.as_bool()?,
            })
        })
        .collect();

    Ok(images)
}

#[tauri::command]
pub async fn pull_docker_image(image_name: String) -> Result<bool, String> {
    let output = Command::new("docker")
        .args(&["pull", &image_name])
        .output()
        .map_err(|e| format!("Failed to execute docker pull command: {}", e))?;

    if output.status.success() {
        Ok(true)
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}


