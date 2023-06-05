import { performance } from "perf_hooks";
import * as vscode from "vscode";

// TODO: Support nested uses, e.g.
//   use contest_lib_rs::{io, emitln, graph::{VertexId, Graph}};
function simplifyUseStatement(line: string): string[] {
  const INDENTATION_RE = /^(\s*)/;
  const SINGLE_USE_RE = /use contest_lib_rs::([\w:]+);/;
  const MULTI_USE_RE = /use contest_lib_rs::\{(.*)\};/;

  const trimmedLine = line.trim();
  const indentation = INDENTATION_RE.exec(line)![1];
  let uses = [];
  let match;

  if ((match = trimmedLine.match(SINGLE_USE_RE))) {
    uses.push(match[1]);
  } else if ((match = trimmedLine.match(MULTI_USE_RE))) {
    uses.push(...match[1].split(",").map((s) => s.trim()));
  } else {
    return [line];
  }

  const nestedUses = uses.filter((s) => s.includes("::"));
  if (nestedUses.length === 0) {
    return [];
  } else if (nestedUses.length === 1) {
    return [`${indentation}use ${nestedUses[0]};`];
  } else {
    return [`${indentation}use {${nestedUses.join(", ")}};`];
  }
}

// Called the very first time a command is executed.
export function activate(context: vscode.ExtensionContext) {
  // TODO: Support multiline use statements and macro definitions.
  // Better yet: proper Rust parser.
  const USE_MODULE_RE = [
    /use contest_lib_rs::\{(\w+)(?:::\w+)*(?:, *(\w+)(?:::\w+)*)*\}/g,
    /use contest_lib_rs::(\w+)/g,
  ];
  const USE_DETECT_RE = /^use contest_lib_rs::/g;
  const MACRO_RE = /macro_rules! +(\w+)/g;

  // TODO: Strip tests as well.
  const SKIP_LINE_RE = /^(\/\/.*)?$/g;

  const RUST_SUFFIX = ".rs";

  let disposable = vscode.commands.registerCommand(
    "rust-collater.collate",
    async () => {
      const startTime = performance.now();

      const editor = vscode.window.activeTextEditor;
      if (!editor) {
        return;
      }

      try {
        // Discard last four path components:
        //   contests/<contest_name>/src/<problem_name>.rs
        const rootDir = vscode.Uri.joinPath(editor.document.uri, "../../../..");
        const srcDir = vscode.Uri.joinPath(rootDir, "src");

        // Improvement potential: Cache macro mapping.
        // Improvement potential: Cache file content. (Check if VSCode does this already.)
        const macroDefinitions = new Map<string, string>();
        const srcFiles = await vscode.workspace.fs.readDirectory(srcDir);
        for (const [fileName, fileType] of srcFiles) {
          if (fileType !== vscode.FileType.File) {
            continue;
          }
          if (!fileName.endsWith(RUST_SUFFIX)) {
            continue;
          }
          const moduleName = fileName.slice(0, -RUST_SUFFIX.length);
          const fileData = await vscode.workspace.fs.readFile(
            vscode.Uri.joinPath(srcDir, fileName)
          );
          const fileText = Buffer.from(fileData).toString("utf8");
          for (const match of fileText.matchAll(MACRO_RE)) {
            macroDefinitions.set(match[1], moduleName);
          }
        }

        let currentText = editor.document.getText();
        const uses: string[] = [];
        for (const re of USE_MODULE_RE) {
          for (const match of currentText.matchAll(re)) {
            uses.push(...match.slice(1));
          }
        }
        // vscode.window.showInformationMessage(uses.join(", "));

        const modulesToInclude = new Set<string>();
        for (const use of uses) {
          const macroOwner = macroDefinitions.get(use);
          if (macroOwner) {
            modulesToInclude.add(macroOwner);
          } else {
            modulesToInclude.add(use);
          }
        }
        // vscode.window.showInformationMessage(
        //   Array.from(modules.values()).join(", ")
        // );

        let outputText = currentText
          .split("\n")
          .flatMap(simplifyUseStatement)
          .join("\n");
        for (const moduleName of modulesToInclude) {
          const fileName = moduleName + RUST_SUFFIX;
          const fileData = await vscode.workspace.fs.readFile(
            vscode.Uri.joinPath(srcDir, fileName)
          );
          const fileText = Buffer.from(fileData).toString("utf8");
          const compressedText = fileText
            .split("\n")
            .filter((line) => line.trim().match(SKIP_LINE_RE) === null)
            .join("\n");
          outputText += `\nmod ${moduleName} {\n${compressedText}\n} // ${moduleName}\n`;
        }
        vscode.env.clipboard.writeText(outputText.trimStart());

        const timeSpent = (performance.now() - startTime) / 1000.0;
        vscode.window.showInformationMessage(
          `Collated to clipboard (${timeSpent.toFixed(2)} s).`
        );
      } catch (e) {
        // Improvement potential: Better error messages:
        //   - for missing src directory;
        //   - for missing module (also highlight the `use` statement)
        vscode.window.showErrorMessage(`Collation error: ${e}`);
      }
    }
  );

  context.subscriptions.push(disposable);
}

// Called when the extension is deactivated.
export function deactivate() {}
