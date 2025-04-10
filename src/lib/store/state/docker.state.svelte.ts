import type {
	DockerContainerInterface,
	DockerImageInterface
} from '$lib/model/interface/docker.interface';

export const dockerStatusState = $state({
	status: false
});

export const dockerContainersState = $state({
	containers: <DockerContainerInterface[]>[]
});

export const dockerImagesState = $state({
	images: <DockerImageInterface[]>[]
});
