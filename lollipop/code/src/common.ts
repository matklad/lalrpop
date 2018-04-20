import * as vscode from 'vscode'
import { log } from 'util'

export function createPlugin(
    backend,
    disposables: vscode.Disposable[],
    doHighlighting: boolean = false,
    diganosticCollection: vscode.DiagnosticCollection | null = null
) {
    let fileExtension = "lollipop"
    let uris = {
        syntaxTree: vscode.Uri.parse("lollipop://syntaxtree")
    }
    let textDocumentContentProvider = new TextDocumentContentProvider(currentFile, uris)

    let getFile = documentToFile(backend, fileExtension, disposables, (doc) => {
        let emitter = textDocumentContentProvider.eventEmitter
        emitter.fire(uris.syntaxTree)
        updateActiveEditor()
    })

    function updateActiveEditor() {
        let editor = vscode.window.activeTextEditor
        if (editor == null) return
        let file = currentFile()
        if (file == null) return
        if (doHighlighting) {
            setHighlights(editor, file.highlight())
        }
        if (diganosticCollection != null) {
            diganosticCollection.clear()
//            diganosticCollection.set(
//                editor.document.uri,
//                file.diagnostics()
//            )
        }
    }


    function currentFile(): EditorFile | null {
        let editor = vscode.window.activeTextEditor
        if (editor == null) return
        let doc = editor.document
        return getFile(doc)
    }

    vscode.window.onDidChangeActiveTextEditor(updateActiveEditor)

    return {
        getFile: getFile,
        showSyntaxTree: () => {
            let file = currentFile()
            if (file == null) return
            return openDoc(uris.syntaxTree)
        },
        extendSelection: () => {
            let editor = vscode.window.activeTextEditor
            let file = currentFile()
            if (editor == null || file == null) return
            editor.selections = editor.selections.map((s) => {
                let range = file.extendSelection(s)
                return new vscode.Selection(range.start, range.end)
            })
        },
        textDocumentContentProvider: textDocumentContentProvider,
    }
}


export interface FileStructureNode {
    name: string
    range: [number, number]
    children: [FileStructureNode]
}

export class EditorFile {
    backend;
    imp;
    doc: vscode.TextDocument;

    constructor(backend, imp, doc: vscode.TextDocument) {
        this.backend = backend
        this.imp = imp
        this.doc = doc
    }

    syntaxTree(): string { return this.call("syntaxTree") }
    extendSelection(range_: vscode.Range): vscode.Range | null {
        let range = fromVsRange(this.doc, range_)
        let exp = this.call("extendSelection", range)
        if (exp == null) return null
        return toVsRange(this.doc, exp)
    }

    structure(): Array<FileStructureNode> { return this.call("structure") }

    highlight(): Array<[[number, number], string]> { return this.call("highlight") }

    call(method: string, ...args) {
        log(`${method} ${args}`)
        let result = this.backend[method](this.imp, ...args)
        log(`${result}`)
        return result
    }
}

function documentToFile(backend, fileExtension: string, disposables: vscode.Disposable[], onChange) {
    let docs = {}
    function update(doc: vscode.TextDocument, file) {
        let key = doc.uri.toString()
        if (file == null) {
            delete docs[key]
        } else {
            docs[key] = file
        }
        onChange(doc)
    }
    function get(doc: vscode.TextDocument) {
        return docs[doc.uri.toString()]
    }

    function isKnownDoc(doc: vscode.TextDocument) {
        return doc.fileName.endsWith(`.${fileExtension}`)
    }

    vscode.workspace.onDidChangeTextDocument((event: vscode.TextDocumentChangeEvent) => {
        let doc = event.document
        if (!isKnownDoc(event.document)) return
        let tree = get(doc)
        update(doc, null)
    }, null, disposables)

    vscode.workspace.onDidOpenTextDocument((doc: vscode.TextDocument) => {
        if (!isKnownDoc(doc)) return
        update(doc, backend.parse(doc.getText()))
    }, null, disposables)

    vscode.workspace.onDidCloseTextDocument((doc: vscode.TextDocument) => {
        update(doc, null)
    }, null, disposables)

    return (doc: vscode.TextDocument) => {
        if (!isKnownDoc(doc)) return null

        if (!get(doc)) {
            update(doc, backend.parse(doc.getText()))
        }
        let imp = get(doc)
        return new EditorFile(backend, imp, doc)
    }
}

export class TextDocumentContentProvider implements vscode.TextDocumentContentProvider {
    uris
    currentFile: () => EditorFile | null;
    constructor(currentFile, uris) {
        this.currentFile = currentFile
        this.uris = uris
    }

    public eventEmitter = new vscode.EventEmitter<vscode.Uri>()
    public syntaxTree: string = "Not available"

    public provideTextDocumentContent(uri: vscode.Uri): vscode.ProviderResult<string> {
        let file = this.currentFile()
        if (file == null) return
        if (uri.toString() == this.uris.syntaxTree.toString()) {
            return file.syntaxTree()
        }
        log(`Bad uri: ${uri}`)
    }

    get onDidChange(): vscode.Event<vscode.Uri> {
        return this.eventEmitter.event
    }
}


const decorations = (() => {
    const decor = (obj) => vscode.window.createTextEditorDecorationType({ color: obj })
    return {
        background: decor("#3F3F3F"),
        error: vscode.window.createTextEditorDecorationType({
            borderColor: "red",
            borderStyle: "none none dashed none",
        }),
        comment: decor("#7F9F7F"),
        string: decor("#CC9393"),
        keyword: decor("#F0DFAF"),
        function: decor("#93E0E3"),
        parameter: decor("#94BFF3"),
        builtin: decor("#DD6718"),
        text: decor("#DCDCCC"),
        attribute: decor("#BFEBBF"),
        literal: decor("#DFAF8F"),
    }
})()

function setHighlights(
    editor: vscode.TextEditor,
    highlihgs: Array<[[number, number], string]>
) {
    let byTag = {}
    for (let tag in decorations) {
        byTag[tag] = []
    }

    for (let [_range, tag] of highlihgs) {
        if (!byTag[tag]) {
            log(`unknown tag ${tag}`)
            continue
        }
        let range = toVsRange(editor.document, _range)
        byTag[tag].push(range)
    }

    for (let tag in byTag) {
        let dec = decorations[tag]
        let ranges = byTag[tag]
        editor.setDecorations(dec, ranges)
    }
}

export function toVsRange(doc: vscode.TextDocument, range: [number, number]): vscode.Range {
    return new vscode.Range(
        doc.positionAt(range[0]),
        doc.positionAt(range[1]),
    )
}

function fromVsRange(doc: vscode.TextDocument, range: vscode.Range): [number, number] {
    return [doc.offsetAt(range.start), doc.offsetAt(range.end)]
}

export function toVsEdits(doc: vscode.TextDocument, edits): Array<vscode.TextEdit> {
    return edits.map((op) => vscode.TextEdit.replace(toVsRange(doc, op.delete), op.insert))
}

async function openDoc(uri: vscode.Uri) {
    let document = await vscode.workspace.openTextDocument(uri)
    vscode.window.showTextDocument(document, vscode.ViewColumn.Two, true)
}
