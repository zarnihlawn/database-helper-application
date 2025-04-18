import type { DatabaseConnectionInterface } from '$lib/model/interface/schema.interface';

export const selectedDatabaseState = $state({
	selectedDatabase: null as DatabaseConnectionInterface | null
});
