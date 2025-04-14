import { sqliteTable, text, integer } from 'drizzle-orm/sqlite-core';

export const userAuthenticationTypeTable = sqliteTable('user_authentication_type', {
	id: integer('id').unique().primaryKey({ autoIncrement: true }),
	type: text('type', { length: 25 }).notNull(),
	description: text('description', { length: 100 }).notNull()
});

export const userTable = sqliteTable('user', {
	id: integer('id').unique().primaryKey({ autoIncrement: true }),
	authentication_type_id: integer('authentication_type_id')
		.notNull()
		.references(() => userAuthenticationTypeTable.id),
	name: text('name', { length: 50 }).notNull(),
	password: text('password', { length: 50 }).notNull(),
	email: text('email', { length: 100 }).unique().notNull(),
	secondary_email: text('secondary_email', { length: 100 })
});

export const userPreviousPasswordTable = sqliteTable('user_previous_password', {
	id: integer('id').unique().primaryKey({ autoIncrement: true }),
	user_id: integer('user_id')
		.notNull()
		.references(() => userTable.id),
	password: text('password', { length: 50 }).notNull()
});

export const datasourceTable = sqliteTable('datasource', {
	id: integer('id').unique().primaryKey({ autoIncrement: true }),
	name: text('type', { length: 25 }).unique().notNull(),
	description: text('description', { length: 100 }).notNull()
});

export const datasourceAuthenticationTypeTable = sqliteTable('datasource_authentication_type', {
	id: integer('id').unique().primaryKey({ autoIncrement: true }),
	type: text('type', { length: 25 }).unique().notNull(),
	description: text('description', { length: 100 }).notNull()
});

export const datasourceConnectionTable = sqliteTable('datasource_connection', {
	id: integer('id').unique().primaryKey({ autoIncrement: true }),
	datasource_id: integer('datasource_id')
		.notNull()
		.references(() => datasourceTable.id),
	datasource_authentication_type_id: integer('datasource_authentication_type_id').references(
		() => datasourceAuthenticationTypeTable.id
	),
	connection_name: text('connection_name', { length: 50 }),
	host: text('host'),
	port: integer('port'),
	username: text('username'),
	password: text('password'),
	driver: text('driver'),
	sid: text('sid'),
	url: text('url'),
	path: text('path')
});

export const databaseTable = sqliteTable('database', {
	id: integer('id').unique().primaryKey({ autoIncrement: true }),
	datasource_connection_id: integer('datasource_connection_id')
		.notNull()
		.references(() => datasourceConnectionTable.id),
	user_id: integer('user_id')
		.notNull()
		.references(() => userTable.id)
});

export const contentTypeTable = sqliteTable('content_type', {
	id: integer('id').unique().primaryKey({ autoIncrement: true }),
	name: text('name', { length: 50 }).unique().notNull(),
	description: text('description', { length: 100 }).notNull()
});

export const queryFileTable = sqliteTable('query_file', {
	id: integer('id').unique().primaryKey({ autoIncrement: true }),
	name: text('name', { length: 20 }).notNull(),
	description: text('description', { length: 100 }).notNull()
});

export const queryBlockTable = sqliteTable('query_block', {
	id: integer('id').unique().primaryKey({ autoIncrement: true }),
	query_file_id: integer('query_file_id')
		.notNull()
		.references(() => queryFileTable.id),
	content_type_id: integer('content_type_id')
		.notNull()
		.references(() => contentTypeTable.id),
	serial_order: integer('serial_order'),
	query_content_block: text('query_content_block')
});
