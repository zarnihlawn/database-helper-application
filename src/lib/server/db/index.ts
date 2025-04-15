import { drizzle } from 'drizzle-orm/libsql';
import { createClient } from '@libsql/client';
import * as schema from './schema';
import * as os from 'os';
import * as path from 'path';

function getDbPath(): string {
	const homeDir = os.homedir();
	return path.join(homeDir, '.config', 'database-helper-application', 'local.db');
}

const databasePath = getDbPath();

const databaseUrl = `file:${databasePath}`;

const client = createClient({ url: databaseUrl });

export const db = drizzle(client, { schema });
