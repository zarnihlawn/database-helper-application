import { sqliteTable, text, integer } from 'drizzle-orm/sqlite-core';

export const user_authorization_level = sqliteTable('user_authorization_level', {
	id: integer('id').unique().primaryKey({ autoIncrement: true }),
	level: integer('level').unique().notNull(),
	description: text('description', {length: 100}).notNull()
});

export const user_authentication_type = sqliteTable('user_authentication_type', {
	id: integer('id').unique().primaryKey({ autoIncrement: true }),
	type: text('type', {length: 25}).notNull(),
	description: text('description', {length: 100}).notNull()
});

export const user = sqliteTable('user', {
	id: integer('id').unique().primaryKey({ autoIncrement: true }),
	authorization_level_id: integer('authorization_level_id').notNull().references(() => user_authorization_level.id),
	authentication_type_id: integer('authentication_type_id').notNull().references(() => user_authentication_type.id),
	name: text('name', {length: 50}).notNull(),
	password: text('password', {length: 50}).notNull(),
	email: text('email', {length: 100}).unique().notNull(),
	secondary_email: text('secondary_email', {length: 100}),
});

export const user_previous_password = sqliteTable('user_previous_password', {
	id: integer('id').unique().primaryKey({ autoIncrement: true }),
	user_id: integer('user_id').notNull().references(() => user.id),
	password: text('password', {length: 50}).notNull(),
});

export const datasource = sqliteTable('datasource', {
	id: integer('id').unique().primaryKey({ autoIncrement: true }),
	name: text('type', {length: 25}).unique().notNull(),
	description: text('description', {length: 100}).notNull()
});


export const datasource_authentication_type = sqliteTable('datasource_authentication_type', {
	id: integer('id').unique().primaryKey({ autoIncrement: true }),
	type: text('type', {length: 25}).unique().notNull(),
	description: text('description', {length: 100}).notNull()
});

export const datasource_connection = sqliteTable('datasource_connection', {
	id: integer('id').unique().primaryKey({ autoIncrement: true }),
	datasource_id: integer('datasource_id').notNull().references(() => datasource.id),
	datasource_authentication_type_id: integer('datasource_authentication_type_id').references(() => datasource_authentication_type.id),
	connection_name: text('connection_name', {length: 50}),
	host: text('host'),
	port: integer('port'),
	username: text('username'),
	password: text('password'),
	driver: text('driver'),
	sid: text('sid'),
	url: text('url'),
	path: text('path'),
});

export const database = sqliteTable('database', {
	id: integer('id').unique().primaryKey({ autoIncrement: true }),
	datasource_connection_id: integer('datasource_connection_id').notNull().references(() => datasource_connection.id),
	user_id: integer('user_id').notNull().references(() => user.id),
});

export const content_type = sqliteTable('content_type', {
	id: integer('id').unique().primaryKey({ autoIncrement: true }),
	name: text('name', {length: 50}).unique().notNull(),
	description: text('description', {length: 100}).notNull()
});

export const query_file = sqliteTable('query_file', {
	id: integer('id').unique().primaryKey({ autoIncrement: true }),
	name: text('name', {length: 20}).notNull(),
	description: text('description', {length: 100}).notNull()
});

export const query_block = sqliteTable('query_block', {
	id: integer('id').unique().primaryKey({ autoIncrement: true }),
	query_file_id: integer('query_file_id').notNull().references(() => query_file.id),
	content_type_id: integer('content_type_id').notNull().references(() => content_type.id),
	serial_order: integer('serial_order'),
	query_content_block: text('query_content_block')
});
