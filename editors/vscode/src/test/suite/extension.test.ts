import * as assert from 'assert';
import * as vscode from 'vscode';
import { extractKmlVersion, getVersionLine, isCompatible } from '../../extension';

const extensionId = 'HiroyukiFuruno.vscode-katana-markdown-linter';

export async function run(): Promise<void> {
	const extension = vscode.extensions.getExtension(extensionId);
	assert.ok(extension, 'Extension should be present.');

	await extension.activate();
	assert.strictEqual(extension.isActive, true, 'Extension should activate.');

	assert.strictEqual(extractKmlVersion('0.18.3'), '0.18.3');
	assert.strictEqual(extractKmlVersion('kml 0.18.3'), '0.18.3');
	assert.strictEqual(extractKmlVersion('katana-markdown-linter 0.18.3'), '0.18.3');
	assert.strictEqual(extractKmlVersion('not a version'), null);

	const extensionVersion = String(extension.packageJSON.version);
	const versionLine = getVersionLine(extensionVersion);
	assert.ok(versionLine, 'Extension package version should have a major and minor version.');
	assert.strictEqual(isCompatible(extensionVersion, extensionVersion), true);
	assert.strictEqual(isCompatible(`kml ${versionLine}.0`, extensionVersion), true);
	assert.strictEqual(isCompatible('invalid', extensionVersion), false);

	const [major, minor] = versionLine.split('.').map(Number);
	assert.strictEqual(isCompatible(`${major}.${minor + 1}.0`, extensionVersion), false);
	if (minor > 0) {
		assert.strictEqual(isCompatible(`${major}.${minor - 1}.10`, extensionVersion), false);
	}
}
