import type { ContentTypeInterface } from '$lib/model/interface/schema.interface';

export const contentTypesData: ContentTypeInterface[] = [
	{ id: 1, name: 'markdown', description: 'A Markdown content type.' },
	{ id: 2, name: 'json', description: 'A JSON content type.' },
	{ id: 3, name: 'sql', description: 'AN SQL content type.' }
];
