'use strict'

import * as vscode from 'vscode'
import { createPlugin, EditorFile, toVsRange } from './common'
import { log } from 'util'

const backend = require("../../native")

const lollipopDiagnostics = vscode.languages.createDiagnosticCollection("lollipop")

export function activate(context: vscode.ExtensionContext) {
    let plugin = createPlugin(backend, context.subscriptions, true, lollipopDiagnostics)

    let commands = [
        ["lollipop.semanticSelection", "extendSelection"],
        ["lollipop.showSyntaxTree", "showSyntaxTree"],
    ]
    for (let [key, action] of commands) {
        let cmd = vscode.commands.registerCommand(key, () => plugin[action]())
        context.subscriptions.push(cmd)
    }
    let providers = [
        vscode.workspace.registerTextDocumentContentProvider('lollipop', plugin.textDocumentContentProvider)
    ]
    context.subscriptions.push(...providers)
}


export function deactivate() { }
