import { db } from '$lib/server/db';
import {
	contentTypeTable,
	datasourceAuthenticationTypeTable,
	datasourceTable,
	userAuthenticationTypeTable,
	userAuthorizationLevelTable,
	userTable
} from '$lib/server/db/schema';

export async function initDatabaseWithDefaultData() {
	const userAuthenticationType = await db.query.userAuthenticationTypeTable.findMany();
	const userAuthorizationLevel = await db.query.userAuthorizationLevelTable.findMany();

	const dataSource = await db.query.datasourceTable.findMany();
	const dataSourceAuthenticationType = await db.query.datasourceAuthenticationTypeTable.findMany();

	const user = await db.query.userTable.findMany();

	const contentType = await db.query.contentTypeTable.findMany();

	if (userAuthenticationType.length === 0 || userAuthenticationType.length < 2) {
		await db.insert(userAuthenticationTypeTable).values([
			{
				type: 'Guest',
				description: 'Guest'
			},
			{
				type: 'Email and Password',
				description: 'Simple email and password authentication'
			}
		]);
	}

	if (userAuthorizationLevel.length === 0 || userAuthorizationLevel.length < 3) {
		await db.insert(userAuthorizationLevelTable).values([
			{
				level: 0,
				description: 'Guest: View data and reports with read-only access.'
			},
			{
				level: 1,
				description:
					'User: Full control over your own data connections: view, edit, delete, report, and export.'
			},
			{
				level: 2,
				description:
					'Authorized: Complete system access: view, edit, delete, export, and collaborate across all data.'
			}
		]);
	}

	if (dataSource.length === 0 || dataSource.length < 5) {
		await db.insert(datasourceTable).values([
			{
				name: 'MongoDB',
				description:
					'A flexible NoSQL document database designed for scalable and high-performance applications.'
			},
			{
				name: 'MySQL',
				description:
					'A widely used relational database managing data in structured tables for diverse applications.'
			},
			{
				name: 'PostgreSQL',
				description:
					'A powerful, open-source relational database known for its reliability and advanced features.'
			},
			{
				name: 'SQLite',
				description:
					'A lightweight, self-contained relational database ideal for embedded systems and local storage.'
			},
			{
				name: 'Oracle',
				description:
					'A robust enterprise-grade relational database optimized for complex data management and high availability.'
			}
		]);
	}

	if (dataSourceAuthenticationType.length === 0 || dataSourceAuthenticationType.length < 1) {
		await db.insert(datasourceAuthenticationTypeTable).values([
			{
				type: 'Username and Password',
				description: 'Simple username and password authentication'
			}
		]);
	}

	if (user.length === 0) {
		await db.insert(userTable).values([
			{
				name: 'Guest',
				email: 'guest@example.com',
				password: 'guest',
				authorization_level_id: 1,
				authentication_type_id: 1
			}
		]);
	}

	if (contentType.length === 0 && contentType.length < 3) {
		await db.insert(contentTypeTable).values([
			{
				name: 'JSON',
				description: 'JSON content type'
			},
			{
				name: 'Markdown',
				description: 'Markdown content type'
			},
      {
        name: 'SQL',
        description: 'SQL content type'
      }
		]);
	}
}
