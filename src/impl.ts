import * as vscode from 'vscode';
import * as path from 'path';
import { TextEditor, Position, Range, Selection, TextDocument } from 'vscode';
import { show } from './utils';
import { FileSource, FilterCommand, FilterPatternType, FilterParam, Singleton } from './schema';
const addon = require('bindings')('addon');


function get_all_text_selection(doc: TextDocument): Range {
	const start = new Position(0, 0);
	const end = new Position(doc.lineCount - 1, doc.lineAt(doc.lineCount - 1).text.length);
	return new Range(start, end);
}

export function all_text_in_editor(textEditor: TextEditor): string {
	const doc = textEditor.document;
	const selection = get_all_text_selection(doc);
	let text = doc.getText(selection);
	return text;
}

export function selection_text_in_editor(textEditor: TextEditor) {
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

export function decodeXlog(filePath: string) {
	if (typeof filePath === 'undefined') {
		return;
	}
	// let ext = ".xlog";
	// let dir = path.dirname(filePath);
	// let name = path.basename(filePath, ext);
	// let suffix = (new Date()).getTime();
	// name += "_" + suffix;

	// let dst_path = path.format({
	// 	dir: dir,
	// 	name: name,
	// 	ext: ".xlog.log",
	// });

	// console.log("dir\t" + dir);
	// console.log("name\t" + name);
	// console.log("suffix\t" + suffix);
	// console.log("dst_path\t" + dst_path);

	let dst_path = filePath + ".log";

	console.log("before log\t" + new Date());

	var v = addon.parse_xlog_to_file(filePath, dst_path);
	console.log(v);
	console.log("after log\t" + new Date());

	if (v === "everything is ok") {

		// let fileUri = vscode.Uri.file(v[1]).with({ scheme: 'file' });
		let fileUri = vscode.Uri.file(dst_path).with({ scheme: 'file' });
		show(fileUri);
		try {
			vscode.commands.executeCommand('vscode.open', fileUri);
		}
		catch (error) {
			throw error(error);
		}

	}
}


function increase_filter_log_version(basename: string): string {
	const r = /.(\d*)$/g;
	const matches = r.exec(basename);

	if (matches === undefined || (matches as RegExpExecArray).length < 2) {
		return basename + ".1";
	} else {
		let old_version = (matches as RegExpExecArray)[1];
		let new_version = 1 + Number(old_version);
		let newbasename = basename.replace(r, '.' + new_version);

		return newbasename;
	}
}

export function gen_filter_uri(command: FilterCommand, pattern: string, patternType: FilterPatternType, fileSource: FileSource, fileUri: vscode.Uri): vscode.Uri {
	let ext = '.xlog.log';
	let filePath = fileUri.fsPath;
	let source;


	let dir = path.dirname(filePath);
	let base = path.basename(filePath, ext);

	console.log("base\t" + base);


	if (fileSource === FileSource.virtual) {
		console.log("fileSource === FileSource.virtual\t" + fileUri);
		let editor = vscode.window.activeTextEditor;
		source = editor;
		base = increase_filter_log_version(base);
		console.log("base\t" + base);
	} else {
		source = filePath;
		base += ".1";
		console.log("base\t" + base);
	}
	let newPath = path.format({
		dir: dir,
		name: base,
		ext: ext
	});
	fileUri = vscode.Uri.file(newPath).with({ scheme: "xlogfilter" });

	let para: FilterParam = {
		command: command,
		patternType: patternType,
		pattern: pattern,
		fileSource: fileSource,
		source: source,
	};

	Singleton.getInstance().filterParam = para;


	return fileUri;
}

export function filterCommandProvder(command: FilterCommand, patternType: FilterPatternType): (uri: vscode.Uri) => void {
	console.log("filterCommandProvder");

	let command_impl = (fileUri: vscode.Uri) => {
		if (fileUri === undefined) {
			console.log("fileUri === undefined");

			let editor = vscode.window.activeTextEditor;

			if (editor === undefined) {
				console.log("editor === undefined");

				return;
			}
			else {
				console.log("fileUri !== undefined");

				fileUri = editor.document.uri;;
			}
		}

		let fileSource = FileSource.unknown;

		if (fileUri.scheme === 'file') {
			fileSource = FileSource.file;
		} else if (fileUri.scheme === 'xlogfilter') {
			fileSource = FileSource.virtual;
		} else {
			throw new Error("shit happen\t" + fileUri);
		}
		console.log("command_impl");
		console.log("fileSource" + fileSource);

		const makeInputStr = (pattern: string | undefined) => {
			if (pattern === undefined || pattern === '') {
				console.log('No input');
				return;
			}
			console.log("makeInputStr");

			let uri = gen_filter_uri(command, pattern, patternType, fileSource, fileUri);
			console.log("filterCommandProvder uri\t" + uri);


			vscode.commands.executeCommand('vscode.open', uri);
		};


		vscode.window.showInputBox().then(makeInputStr);
	};

	return command_impl;

}	