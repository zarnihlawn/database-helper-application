export interface ContentTypeInterface {
	id: number;
	name: string;
	description: string;
}

export interface DatabaseConnectionInterface {
	id: number;
	user_id?: number;
	datasource_id: number;
	connection_name: string;
	url: string;
}

export interface DatabaseFileCollectionInterface {
	id: number;
	database_connection_id: number;
	query_file_id: number;
}

export interface DatasourceInterface {
	id: number;
	name: string;
	description: string;
}

export interface QueryBlockInterface {
	id: number;
	query_file_id: number;
	content_type_id: number;
	serial_order: number;
	query_content_block: string;
}

export interface QueryFileInterface {
	id: number;
	name: string;
	description?: string;
}

export interface UserInterface {
	id: number;
	authentication_type_id: number;
	name: string;
	password: string;
	email: string;
	secondary_email?: string;
}

export interface UserCookieInterface {
	id: number;
	name: string;
	email: string;
}
