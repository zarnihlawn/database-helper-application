export interface DatabaseMetadata {
	schemas: SchemaInfo[];
	tables: TableInfo[];
	views: ViewInfo[];
	functions: FunctionInfo[];
	sequences: SequenceInfo[];
	enums: EnumInfo[];
	extensions: ExtensionInfo[];
}

export interface SchemaInfo {
	name: string;
	owner: string;
}

export interface TableInfo {
	name: string;
	columns: ColumnInfo[];
}

export interface ColumnInfo {
	name: string;
	data_type: string;
}

export interface ViewInfo {
	schema: string;
	name: string;
	definition: string;
}

export interface FunctionInfo {
	schema: string;
	name: string;
	return_type: string;
	arguments: string;
	definition: string;
}

export interface SequenceInfo {
	schema: string;
	name: string;
	data_type: string;
	start_value: number;
	increment: number;
	max_value: number;
	min_value: number;
}

export interface EnumInfo {
	schema: string;
	name: string;
	values: string[];
}

export interface ExtensionInfo {
	name: string;
	version: string;
}
