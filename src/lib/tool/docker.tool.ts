import type { DockerContainerInterface, DockerHubImageInterface, DockerImageInterface } from '$lib/model/interface/docker.interface';
import { invoke } from '@tauri-apps/api/core';

export function checkDockerStatus() {
	const status = invoke('check_docker_status');
	return status;
}

export function startDocker() {
	const status = invoke('start_docker_service');
	return status;
}

export function stopDocker() {
	const status = invoke('stop_docker_service');
	return status;
}

export function restartDocker() {
	const status = invoke('restart_docker_service');
	return status;
}

export async function catchAllDockerContainers(): Promise<DockerContainerInterface[]> {
	try {
		const result = await invoke<DockerContainerInterface[]>('get_all_docker_containers');
		return result;
	} catch (error) {
		console.error('Error fetching Docker containers:', error);
		throw error;
	}
}

export async function catchAllDockerImages(): Promise<DockerImageInterface[]> {
	try {
		const result = await invoke<DockerImageInterface[]>('get_all_docker_images');
		return result;
	} catch (error) {
		console.error('Error fetching Docker images:', error);
		throw error;
	}
}

export function startDockerContainer(containerId: string) {
	const status = invoke('start_docker_container', { containerId: containerId });
	return status;
}

export function stopDockerContainer(containerId: string) {
	const status = invoke('stop_docker_container', { containerId: containerId });
	return status;
}

export function deleteDockerContainer(containerId: string) {
	const status = invoke('delete_docker_container', { containerId: containerId });
	return status;
}

export function deleteDockerImage(imageId: string) {
	const status = invoke('delete_docker_image', { imageId: imageId });
	return status;
}

export async function searchDockerImage(query: string): Promise<DockerHubImageInterface[]> {
	try {
		const results = await invoke<DockerHubImageInterface[]>('search_docker_image', { query });
		return results;
	} catch (error) {
		console.error('Error searching Docker images:', error);
		throw error;
	}
}

export async function pullDockerImage(imageName: string) {
	const status = await invoke('pull_docker_image', { imageName: imageName });
	return status;
}
