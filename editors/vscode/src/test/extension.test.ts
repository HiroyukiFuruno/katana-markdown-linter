import * as assert from 'assert';
import * as vscode from 'vscode';
import { extractKmlVersion, isCompatible } from '../extension';

suite('Extension Test Suite', () => {
	vscode.window.showInformationMessage('Start all tests.');

	test('Extension should be present', () => {
		assert.ok(vscode.extensions.getExtension('HiroyukiFuruno.vscode-katana-markdown-linter'));
	});

	test('should activate', async () => {
		const ext = vscode.extensions.getExtension('HiroyukiFuruno.vscode-katana-markdown-linter');
		await ext?.activate();
		assert.strictEqual(ext?.isActive, true);
	});

	test('version check should handle incompatible version', async () => {
		assert.strictEqual(extractKmlVersion('0.18.3'), '0.18.3');
		assert.strictEqual(extractKmlVersion('kml 0.18.3'), '0.18.3');
		assert.strictEqual(extractKmlVersion('katana-markdown-linter 0.18.3'), '0.18.3');
		assert.strictEqual(extractKmlVersion('not a version'), null);
		assert.strictEqual(isCompatible('0.18.10'), true);
		assert.strictEqual(isCompatible('kml 0.18.0'), true);
		assert.strictEqual(isCompatible('0.19.0'), false);
		assert.strictEqual(isCompatible('invalid'), false);
	});
});
