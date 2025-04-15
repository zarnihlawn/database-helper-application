import Database from '@tauri-apps/plugin-sql';

export async function getDatasources() {
	try {
		const db = await Database.load('sqlite:///C:/Users/Asus/Desktop/database-helper-application/local.db');
		const result = await db.select('SELECT * FROM datasource');
		// Use db.select for SELECT queries, not db.execute
		return result;
	} catch (e) {
		console.error('Error fetching datasources:', e);
		return [];
	}
}
