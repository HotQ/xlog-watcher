'use strict';

import * as vscode from 'vscode';
import { FileSource, FilterCommand, FilterPatternType, Singleton } from './schema';
import { all_text_in_editor } from './impl';

const addon = require('bindings')('addon');

export default class XlogFilterLogContentProvider implements vscode.TextDocumentContentProvider {
    public provideTextDocumentContent(uri: vscode.Uri): string {
        let para = Singleton.getInstance().filterParam;
        let source;
        if (para.command !== FilterCommand.select || para.patternType !== FilterPatternType.string || para.fileSource !== FileSource.file) {
            if (para.source !== undefined) {
                source = all_text_in_editor(para.source as vscode.TextEditor);
            }
        } else {
            source = para.source;
        }

        let contend = addon.filter_line(para.command, para.patternType, para.pattern, para.fileSource, source);
        return contend;
    }
}
