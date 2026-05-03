const fs = require('fs');
const path = require('path');

function fail(message) {
    console.error(message);
    process.exit(1);
}

function parseJson(filePath) {
    try {
        return JSON.parse(fs.readFileSync(filePath, 'utf8'));
    } catch (error) {
        fail(`Failed to parse JSON: ${filePath}: ${error.message}`);
    }
}

function verifyVSCodeExtension(extensionPath) {
    if (!fs.existsSync(extensionPath)) {
        fail(`Extension path not found: ${extensionPath}`);
    }

    const packageJsonPath = path.join(extensionPath, 'package.json');
    if (!fs.existsSync(packageJsonPath)) {
        fail(`package.json not found in ${extensionPath}`);
    }

    const packageJson = parseJson(packageJsonPath);
    console.log(`Verifying VS Code extension: ${packageJson.name}@${packageJson.version}`);

    const requiredFields = ['name', 'version', 'publisher', 'engines', 'main', 'contributes'];
    for (const field of requiredFields) {
        if (!packageJson[field]) {
            fail(`Missing required field in package.json: ${field}`);
        }
    }

    if (!packageJson.engines.vscode) {
        fail('Missing required field in package.json.engines: vscode');
    }

    // Main entry point check (usually out/extension.js)
    // Note: This check assumes the extension has been compiled.
    const mainPath = path.join(extensionPath, packageJson.main);
    if (!fs.existsSync(mainPath)) {
        fail(`Main entry point not found: ${mainPath}. Did you run npm run compile?`);
    }

    console.log('VS Code extension verification passed.');
}

const targetDir = process.argv[2] || 'editors/vscode';
verifyVSCodeExtension(targetDir);
