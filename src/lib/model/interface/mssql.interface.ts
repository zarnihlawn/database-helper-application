export interface DatabaseInfo {
	name: string;
	schemas: SchemaInfo[];
}

export interface SchemaInfo {
	name: string;
	tables: TableInfo[];
}

export interface TableInfo {
	name: string;
	columns: ColumnInfo[];
}

export interface ColumnInfo {
	name: string;
	data_type: string;
}
