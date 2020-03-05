'use strict';

import * as vscode from 'vscode';

const addon = require('bindings')('addon');


export default class XlogLogHoverProvider {
    public dispose() {

    }
	public provideHover(document:vscode.TextDocument, position:vscode.Position, token:vscode.CancellationToken) :vscode.Hover { 
		const filePath	= document.fileName;
		const word		= document.getText(document.getWordRangeAtPosition(position));
        

		var hoverInfo = "2020-02-22T17:28:07.513+08:00 Debug\n"+word+"\n"+filePath+"\n"+position+"\n[txt](1.txt) \n[链接](http://www.example.com)";
        return new vscode.Hover( hoverInfo );
        
        // var hoverInfo = addon.get_hover_or_some_shit(filePath, position.line, word);
        // return new vscode.Hover( {language: 'xlog.log', value: hoverInfo} );
	}
}
