import * as vscode from 'vscode';
import { show } from './utils';
import { all_text_in_editor,selection_text_in_editor} from './impl';

const addon = require('bindings')('addon');


export function activate(context: vscode.ExtensionContext) {
	console.log('Congratulations, your extension "xlog-watcher" is now active!');

	vscode.commands.registerTextEditorCommand('extension.showText', all_text_in_editor);
	vscode.commands.registerTextEditorCommand('extension.showTextSelection', selection_text_in_editor);

	let disposable = vscode.commands.registerCommand('extension.helloWorld', () => {
		var info1 = addon.parse_xlog_to_file("\\\\?\\E:\\CODER\\xlog-watcher\\compressed.xlog","E:\\CODER\\xlog-watcher\\sdafs.log");
		var info2 = addon.parse_xlog_to_file("E:\\CODER\\xlog-watcher\\compressed.xlog","E:\\CODER\\xlog-watcher\\dfghret.log");
		show("debug_info1: "+info1);
		show("debug_info2: "+info2);
		show(addon.hello() + '!   212414+64252='+addon.add(212414,64252));
	});

	context.subscriptions.push(disposable);
}


export function deactivate() {
	show('extension is deactivated');
}
