export interface DockerContainerInterface {
	id: string;
	name: string;
	image: string;
	status: string;
	state: string;
	created: string;
	ports: string[];
	labels: Record<string, string>;
}

export interface DockerHubImageInterface {
	name: string;
	description: string;
	star_count: number;
	pull_count: number;
	official: boolean;
}

export interface DockerImageInterface {
	repository: string;
	tag: string;
	image_id: string;
	created: string;
	size: string;
	labels: Record<string, string>;
}
