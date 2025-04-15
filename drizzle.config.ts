import { defineConfig } from 'drizzle-kit';

import * as os from 'os';
import * as path from 'path';

function getDbPath(): string {
	const homeDir = os.homedir();
	return path.join(homeDir, '.config', 'database-helper-application', 'local.db');
}

const databasePath = getDbPath();

const databaseUrl = `file:${databasePath}`;

export default defineConfig({
	schema: './src/lib/server/db/schema.ts',
	dbCredentials: { url: databaseUrl },
	verbose: true,
	strict: true,
	dialect: 'sqlite'
});
