CREATE TABLE `content_type` (
	`id` integer PRIMARY KEY AUTOINCREMENT NOT NULL,
	`name` text(50) NOT NULL,
	`description` text(100) NOT NULL
);
--> statement-breakpoint
CREATE UNIQUE INDEX `content_type_id_unique` ON `content_type` (`id`);--> statement-breakpoint
CREATE UNIQUE INDEX `content_type_name_unique` ON `content_type` (`name`);--> statement-breakpoint
CREATE TABLE `database` (
	`id` integer PRIMARY KEY AUTOINCREMENT NOT NULL,
	`datasource_connection_id` integer NOT NULL,
	`user_id` integer NOT NULL,
	FOREIGN KEY (`datasource_connection_id`) REFERENCES `datasource_connection`(`id`) ON UPDATE no action ON DELETE no action,
	FOREIGN KEY (`user_id`) REFERENCES `user`(`id`) ON UPDATE no action ON DELETE no action
);
--> statement-breakpoint
CREATE UNIQUE INDEX `database_id_unique` ON `database` (`id`);--> statement-breakpoint
CREATE TABLE `datasource_authentication_type` (
	`id` integer PRIMARY KEY AUTOINCREMENT NOT NULL,
	`type` text(25) NOT NULL,
	`description` text(100) NOT NULL
);
--> statement-breakpoint
CREATE UNIQUE INDEX `datasource_authentication_type_id_unique` ON `datasource_authentication_type` (`id`);--> statement-breakpoint
CREATE UNIQUE INDEX `datasource_authentication_type_type_unique` ON `datasource_authentication_type` (`type`);--> statement-breakpoint
CREATE TABLE `datasource_connection` (
	`id` integer PRIMARY KEY AUTOINCREMENT NOT NULL,
	`datasource_id` integer NOT NULL,
	`datasource_authentication_type_id` integer,
	`connection_name` text(50),
	`host` text,
	`port` integer,
	`username` text,
	`password` text,
	`driver` text,
	`sid` text,
	`url` text,
	`path` text,
	FOREIGN KEY (`datasource_id`) REFERENCES `datasource`(`id`) ON UPDATE no action ON DELETE no action,
	FOREIGN KEY (`datasource_authentication_type_id`) REFERENCES `datasource_authentication_type`(`id`) ON UPDATE no action ON DELETE no action
);
--> statement-breakpoint
CREATE UNIQUE INDEX `datasource_connection_id_unique` ON `datasource_connection` (`id`);--> statement-breakpoint
CREATE TABLE `datasource` (
	`id` integer PRIMARY KEY AUTOINCREMENT NOT NULL,
	`type` text(25) NOT NULL,
	`description` text(100) NOT NULL
);
--> statement-breakpoint
CREATE UNIQUE INDEX `datasource_id_unique` ON `datasource` (`id`);--> statement-breakpoint
CREATE UNIQUE INDEX `datasource_type_unique` ON `datasource` (`type`);--> statement-breakpoint
CREATE TABLE `query_block` (
	`id` integer PRIMARY KEY AUTOINCREMENT NOT NULL,
	`query_file_id` integer NOT NULL,
	`content_type_id` integer NOT NULL,
	`serial_order` integer,
	`query_content_block` text,
	FOREIGN KEY (`query_file_id`) REFERENCES `query_file`(`id`) ON UPDATE no action ON DELETE no action,
	FOREIGN KEY (`content_type_id`) REFERENCES `content_type`(`id`) ON UPDATE no action ON DELETE no action
);
--> statement-breakpoint
CREATE UNIQUE INDEX `query_block_id_unique` ON `query_block` (`id`);--> statement-breakpoint
CREATE TABLE `query_file` (
	`id` integer PRIMARY KEY AUTOINCREMENT NOT NULL,
	`name` text(20) NOT NULL,
	`description` text(100) NOT NULL
);
--> statement-breakpoint
CREATE UNIQUE INDEX `query_file_id_unique` ON `query_file` (`id`);--> statement-breakpoint
CREATE TABLE `user_authentication_type` (
	`id` integer PRIMARY KEY AUTOINCREMENT NOT NULL,
	`type` text(25) NOT NULL,
	`description` text(100) NOT NULL
);
--> statement-breakpoint
CREATE UNIQUE INDEX `user_authentication_type_id_unique` ON `user_authentication_type` (`id`);--> statement-breakpoint
CREATE TABLE `user_authorization_level` (
	`id` integer PRIMARY KEY AUTOINCREMENT NOT NULL,
	`level` integer NOT NULL,
	`description` text(100) NOT NULL
);
--> statement-breakpoint
CREATE UNIQUE INDEX `user_authorization_level_id_unique` ON `user_authorization_level` (`id`);--> statement-breakpoint
CREATE UNIQUE INDEX `user_authorization_level_level_unique` ON `user_authorization_level` (`level`);--> statement-breakpoint
CREATE TABLE `user_previous_password` (
	`id` integer PRIMARY KEY AUTOINCREMENT NOT NULL,
	`user_id` integer NOT NULL,
	`password` text(50) NOT NULL,
	FOREIGN KEY (`user_id`) REFERENCES `user`(`id`) ON UPDATE no action ON DELETE no action
);
--> statement-breakpoint
CREATE UNIQUE INDEX `user_previous_password_id_unique` ON `user_previous_password` (`id`);--> statement-breakpoint
CREATE TABLE `user` (
	`id` integer PRIMARY KEY AUTOINCREMENT NOT NULL,
	`authorization_level_id` integer NOT NULL,
	`authentication_type_id` integer NOT NULL,
	`name` text(50) NOT NULL,
	`password` text(50) NOT NULL,
	`email` text(100) NOT NULL,
	`secondary_email` text(100),
	FOREIGN KEY (`authorization_level_id`) REFERENCES `user_authorization_level`(`id`) ON UPDATE no action ON DELETE no action,
	FOREIGN KEY (`authentication_type_id`) REFERENCES `user_authentication_type`(`id`) ON UPDATE no action ON DELETE no action
);
--> statement-breakpoint
CREATE UNIQUE INDEX `user_id_unique` ON `user` (`id`);--> statement-breakpoint
CREATE UNIQUE INDEX `user_email_unique` ON `user` (`email`);

