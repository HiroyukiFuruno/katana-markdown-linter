import * as vscode from 'vscode';
import * as path from 'path';
import {
    LanguageClient,
    LanguageClientOptions,
    Executable,
    TransportKind
} from 'vscode-languageclient/node';

let client: LanguageClient;

export async function activate(_context: vscode.ExtensionContext) {
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
