import * as fs from 'fs';
import * as vscode from 'vscode';
import { TextEditor, TextEditorEdit, Position, Range, Selection, TextDocument } from 'vscode';
import { show } from './utils';

const addon = require('bindings')('addon');


function get_all_text_selection(doc: TextDocument): Range {
	const start = new Position(0, 0);
	const end = new Position(doc.lineCount - 1, doc.lineAt(doc.lineCount - 1).text.length);
	return new Range(start, end);
}

export function all_text_in_editor(textEditor: TextEditor, edit: TextEditorEdit, ...args: any[]) {
	const doc = textEditor.document;
	const selection = get_all_text_selection(doc);
	let text = doc.getText(selection);
	show(text);
}

export function selection_text_in_editor(textEditor: TextEditor, edit: TextEditorEdit, ...args: any[]) {
	const doc = textEditor.document;
	show(doc);

	let selection: Selection | Range = textEditor.selection;

	if (selection.isEmpty) {
		selection = get_all_text_selection(doc);
	}
	console.log('selection', selection);
	let text = doc.getText(selection);
	show(text);
}

export function decodeXlog(filePath:string) {
	if (typeof filePath === 'undefined') {
		return;
	}

	console.log("before log\t" + new Date());
	var v = addon.parse_xlog_to_file_tmp(filePath);
	console.log(v);
	console.log("after log\t" + new Date());

	if(v[0]==="everything is ok") {

		let fileUri = vscode.Uri.file(v[1]).with({ scheme: 'file' });
		vscode.commands.executeCommand('vscode.open', fileUri);
	}
}