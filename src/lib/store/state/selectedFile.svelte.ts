import type { QueryFileInterface } from '$lib/model/interface/schema.interface';

export const selectedFileState = $state({
	selectedFile: null as QueryFileInterface | null
});
