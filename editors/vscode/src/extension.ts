import * as vscode from 'vscode';
import * as path from 'path';
import { execFile } from 'child_process';
import {
    LanguageClient,
    LanguageClientOptions,
    Executable,
    TransportKind
} from 'vscode-languageclient/node';

let client: LanguageClient;

async function getKmlVersion(executablePath: string): Promise<string | null> {
    return new Promise((resolve) => {
        execFile(executablePath, ['--version'], (error, stdout, stderr) => {
            if (error) {
                const output = `${stdout}${stdout ? '\n' : ''}${stderr}`.trim();
                resolve(output || null);
                return;
            }

            resolve(stdout.trim());
        });
    });
}

export function extractKmlVersion(versionOutput: string): string | null {
    const match = versionOutput.match(/(\d+\.\d+\.\d+(?:[-+][0-9A-Za-z.-]+)?)/);
    return match ? match[0] : null;
}

export function getVersionLine(versionOutput: string): string | null {
    const version = extractKmlVersion(versionOutput);
    if (!version) {
        return null;
    }

    return version.split('.').slice(0, 2).join('.');
}

export function isCompatible(version: string, extensionVersion: string): boolean {
    const versionLine = getVersionLine(version);
    const extensionVersionLine = getVersionLine(extensionVersion);

    return versionLine !== null && versionLine === extensionVersionLine;
}

export async function activate(context: vscode.ExtensionContext) {
    const extensionVersion = String(context.extension.packageJSON.version);
    const outputChannel = vscode.window.createOutputChannel('KatanA Markdown Linter');
    const config = vscode.workspace.getConfiguration('kml');
    let executablePath = config.get<string>('executablePath') || 'kml';

    // If it's a relative path, resolve it relative to the workspace root if available
    if (executablePath !== 'kml' && !path.isAbsolute(executablePath)) {
        const workspaceFolders = vscode.workspace.workspaceFolders;
        if (workspaceFolders) {
            executablePath = path.resolve(workspaceFolders[0].uri.fsPath, executablePath);
        }
    }

    outputChannel.appendLine(`Checking KatanA Markdown Linter version from: ${executablePath}`);
    const version = await getKmlVersion(executablePath);
    if (version) {
        outputChannel.appendLine(`Found kml version: ${version}`);
        if (!isCompatible(version, extensionVersion)) {
            const versionLine = getVersionLine(extensionVersion);
            const expectedVersion = versionLine ? `^${versionLine}.0` : 'the version declared by this extension';
            vscode.window.showWarningMessage(
                `KatanA Markdown Linter version ${version} might be incompatible with this extension. Expected ${expectedVersion}.`,
                'Open Settings'
            ).then(selection => {
                if (selection === 'Open Settings') {
                    vscode.commands.executeCommand('workbench.action.openSettings', 'kml.executablePath');
                }
            });
        }
    } else {
        outputChannel.appendLine('Could not determine kml version.');
    }

    outputChannel.appendLine(`Starting KatanA Markdown Linter LSP from: ${executablePath}`);

    const serverOptions: Executable = {
        command: executablePath,
        args: ['lsp'],
        options: {
            env: { ...process.env, RUST_LOG: 'info' }
        },
        transport: TransportKind.stdio
    };

    const clientOptions: LanguageClientOptions = {
        documentSelector: [{ scheme: 'file', language: 'markdown' }],
        synchronize: {
            fileEvents: vscode.workspace.createFileSystemWatcher('**/.markdownlint.{json,jsonc}')
        },
        outputChannel: outputChannel
    };

    client = new LanguageClient(
        'kml',
        'KatanA Markdown Linter',
        serverOptions,
        clientOptions
    );

    try {
        await client.start();
        outputChannel.appendLine('KatanA Markdown Linter LSP is ready.');
    } catch (error: any) {
        outputChannel.appendLine(`Failed to start KatanA Markdown Linter LSP: ${error.message}`);
        if (executablePath === 'kml') {
             vscode.window.showErrorMessage(
                `Failed to find 'kml' executable on PATH. Please install KatanA Markdown Linter or configure 'kml.executablePath'.`,
                'Open Settings'
            ).then(selection => {
                if (selection === 'Open Settings') {
                    vscode.commands.executeCommand('workbench.action.openSettings', 'kml.executablePath');
                }
            });
        } else {
             vscode.window.showErrorMessage(
                `Failed to start 'kml' at ${executablePath}. Please check your 'kml.executablePath' setting.`,
                'Open Settings'
            ).then(selection => {
                if (selection === 'Open Settings') {
                    vscode.commands.executeCommand('workbench.action.openSettings', 'kml.executablePath');
                }
            });
        }
    }
}

export function deactivate(): Thenable<void> | undefined {
    if (!client) {
        return undefined;
    }
    return client.stop();
}
