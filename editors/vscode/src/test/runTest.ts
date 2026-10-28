import * as path from 'path';
import { runTests } from '@vscode/test-electron';

async function main(): Promise<void> {
	await runTests({
		version: '1.75.0',
		extensionDevelopmentPath: path.resolve(__dirname, '../..'),
		extensionTestsPath: path.resolve(__dirname, 'suite/index'),
		launchArgs: [
			'--disable-extensions',
			'--disable-gpu',
			'--user-data-dir=/tmp/kml-vscode-test-user-data'
		]
	});
}

main().catch((error: unknown) => {
	console.error('Failed to run VS Code extension tests');
	console.error(error);
	process.exitCode = 1;
});
