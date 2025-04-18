export interface ColumnInfoInterface {
	name: string;
	data_type: string;
}

export interface TableInfoInterface {
	name: string;
	columns: ColumnInfoInterface[];
}
