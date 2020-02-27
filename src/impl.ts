import { TextEditor, TextEditorEdit, Position, Range, Selection, TextDocument } from 'vscode';
import { show } from './utils';

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
