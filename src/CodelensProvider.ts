import * as vscode from "vscode";
import { parse_lines } from "money-parser";

/**
 * CodelensProvider
 */
export class CodelensProvider implements vscode.CodeLensProvider {
  private codeLenses: vscode.CodeLens[] = [];
  private _onDidChangeCodeLenses: vscode.EventEmitter<void> =
    new vscode.EventEmitter<void>();
  public readonly onDidChangeCodeLenses: vscode.Event<void> =
    this._onDidChangeCodeLenses.event;

  constructor() {
    vscode.workspace.onDidChangeConfiguration((_) => {
      this._onDidChangeCodeLenses.fire();
    });
  }

  public provideCodeLenses(
    document: vscode.TextDocument,
    token: vscode.CancellationToken
  ): vscode.CodeLens[] | Thenable<vscode.CodeLens[]> {
    if (
      vscode.workspace
        .getConfiguration("codelens-sample")
        .get("enableCodeLens", true)
    ) {
      this.codeLenses = [];

      const text = document.getText();

      // const print = (inp: any) =>
      // vscode.window.showInformationMessage(inspect(inp));

      const balances: Map<string, number> | null = parse_lines(text);

      if (balances === null) {
        return this.codeLenses;
      }

      const line = document.lineAt(
        document.positionAt(text.lastIndexOf(" ")).line
      );

      this.codeLenses.push(
        new vscode.CodeLens(line.range, {
          title: [...balances.entries()]
            .sort(([a], [b]) => a.localeCompare(b))
            .map((x) => `${x[0]}: ${x[1]}`)
            .join(" | "),
          tooltip: "Tooltip provided by sample extension",
          command: "codelens-sample.codelensAction",
          arguments: ["Argument 1", false],
        })
      );

      return this.codeLenses;
    }
    return [];
  }

  // public resolveCodeLens(
  //   codeLens: vscode.CodeLens,
  //   token: vscode.CancellationToken
  // ) {
  //   if (
  //     vscode.workspace
  //       .getConfiguration("codelens-sample")
  //       .get("enableCodeLens", true)
  //   ) {
  //     codeLens.command = {
  //       title: "Codelens provided by sample extension",
  //       tooltip: "Tooltip provided by sample extension",
  //       command: "codelens-sample.codelensAction",
  //       arguments: ["Argument 1", false],
  //     };
  //     return codeLens;
  //   }
  //   return null;
  // }
}
