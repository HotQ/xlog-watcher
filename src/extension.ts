import * as vscode from 'vscode';
import { show } from './utils';
import { all_text_in_editor, selection_text_in_editor, decodeXlog, filterCommandProvder } from './impl';
import * as path from 'path';
import { FilterCommand, FilterPatternType } from './schema';
import XlogLogHoverProvider from './hoverProvider';
import XlogFilterLogContentProvider from './contentProvider';
const addon = require('bindings')('addon');


export function activate(context: vscode.ExtensionContext) {
	console.log('Congratulations, your extension "xlog-watcher" is now active!');

	vscode.commands.registerTextEditorCommand('extension.showText', all_text_in_editor);
	vscode.commands.registerTextEditorCommand('extension.showTextSelection', selection_text_in_editor);
	vscode.commands.registerCommand('extension.decodeXlog', (fileUri: vscode.Uri) => {
		var filepath = fileUri.fsPath;
		decodeXlog(filepath);
	});

	vscode.commands.registerCommand('extension.selectString', filterCommandProvder(FilterCommand.select, FilterPatternType.string));
	vscode.commands.registerCommand('extension.deleteString', filterCommandProvder(FilterCommand.delete, FilterPatternType.string));

	let disposable = vscode.commands.registerCommand('extension.helloWorld', () => {
		show("hello " + new Date());
		show(addon.hello() + '!   212414+64252=' + addon.add(212414, 64252));
	});

	context.subscriptions.push(disposable);


	let hoverProvider = new XlogLogHoverProvider();
	vscode.languages.registerHoverProvider('log', hoverProvider);
	context.subscriptions.push(hoverProvider);


	let provider = new XlogFilterLogContentProvider();
	vscode.workspace.registerTextDocumentContentProvider('xlogfilter', provider);
}


export function deactivate() {
	show('extension is deactivated');
}
