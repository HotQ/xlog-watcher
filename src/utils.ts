import * as vscode from 'vscode';


export function show(message?: any, ...optionalParams: any[]) {
	vscode.window.showInformationMessage(message,...optionalParams);
	console.log(message,...optionalParams);
}
