import { TextEditor } from 'vscode';

export enum FileSource {
    unknown,
    file,
    virtual,
}

export enum FilterCommand {
    unknown,
    select,
    delete,
}

export enum FilterPatternType {
    unknown,
    string,
    reg,
}

export interface FilterParam {
    command: FilterCommand;
    patternType: FilterPatternType;
    pattern: string;
    fileSource: FileSource;
    source: String | TextEditor | undefined; // if fileSource == file, source show the path, otherwise the string we wanna crop
}

export function init_filter_param(): FilterParam {
    return {
        command: FilterCommand.unknown,
        patternType: FilterPatternType.unknown,
        pattern: "",
        fileSource: FileSource.unknown,
        source: "",
    };
}

export class Singleton {
    private static singleton: Singleton;
    private constructor() { }
    public _filterParam: FilterParam = init_filter_param();

    get filterParam(): FilterParam {
        return this._filterParam;
    }
    set filterParam(value: FilterParam) {
        this._filterParam = value;
    }


    public static getInstance(): Singleton {
        if (!Singleton.singleton) {
            Singleton.singleton = new Singleton();
        }
        return Singleton.singleton;
    }
}