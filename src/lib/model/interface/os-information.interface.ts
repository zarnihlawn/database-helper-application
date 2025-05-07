export interface OsInformation {
	arch?: string;
	exe_extension?: string;
	family?: string;
	hostname?: string;
	locale?: string;
	platform?: string;
	type?: string;
	version?: string;
}

export interface MemoryInformation {
	total_memory?: number;
	available_memory?: number;
}
