import { performance } from "perf_hooks";
import * as vscode from "vscode";

async function fileExists(uri: vscode.Uri): Promise<boolean> {
  try {
    await vscode.workspace.fs.stat(uri);
    return true;
  } catch (e) {
    return false;
  }
}

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
  const MODULE_DEPENDENCY_RE = [
    /use crate::\{(\w+)(?:::\w+)*(?:, *(\w+)(?:::\w+)*)*\}/g,
    /use crate::(\w+)/g,
  ];
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
        // Skip `<contest_name>/src/<problem_name>.rs` and find the next Rust folder.
        let rootDir = vscode.Uri.joinPath(editor.document.uri, "../../..");
        while (
          !(await fileExists(vscode.Uri.joinPath(rootDir, "Cargo.toml")))
        ) {
          rootDir = vscode.Uri.joinPath(rootDir, "..");
        }
        const srcDir = vscode.Uri.joinPath(rootDir, "src");

        const moduleBodies = new Map<string, string>();
        const srcFiles = await vscode.workspace.fs.readDirectory(srcDir);
        for (const [fileName, fileType] of srcFiles) {
          if (fileType !== vscode.FileType.File) {
            continue;
          }
          if (!fileName.endsWith(RUST_SUFFIX)) {
            continue;
          }
          const moduleName = fileName.slice(0, -RUST_SUFFIX.length);
          const moduleData = await vscode.workspace.fs.readFile(
            vscode.Uri.joinPath(srcDir, fileName)
          );
          const moduleText = Buffer.from(moduleData).toString("utf8");
          let moduleBody = "";
          for (const line of moduleText.split("\n")) {
            if (line.trim().match(SKIP_LINE_RE) === null) {
              moduleBody += line + "\n";
            }
          }
          moduleBodies.set(moduleName, moduleBody);
        }

        // Improvement potential: Cache macro mapping.
        // Improvement potential: Cache file content. (Check if VSCode does this already.)
        const macroDefinitions = new Map<string, string>();
        const moduleDependencies = new Map<string, string[]>();
        for (const [moduleName, moduleBody] of moduleBodies.entries()) {
          for (const match of moduleBody.matchAll(MACRO_RE)) {
            macroDefinitions.set(match[1], moduleName);
          }
          let thisModuleDependencies: string[] = [];
          for (const re of MODULE_DEPENDENCY_RE) {
            for (const match of moduleBody.matchAll(re)) {
              thisModuleDependencies.push(...match.slice(1));
            }
          }
          moduleDependencies.set(moduleName, thisModuleDependencies);
        }

        let currentText = editor.document.getText();
        const dependencies: string[] = [];
        for (const re of USE_MODULE_RE) {
          for (const match of currentText.matchAll(re)) {
            dependencies.push(...match.slice(1));
          }
        }

        const modulesToInclude = new Set<string>();
        while (dependencies.length > 0) {
          const dep = dependencies.pop()!;
          const module = macroDefinitions.get(dep) || dep;
          if (!modulesToInclude.has(module)) {
            modulesToInclude.add(module);
            dependencies.push(...(moduleDependencies.get(module) || []));
          }
        }

        let outputText = currentText
          .split("\n")
          .flatMap(simplifyUseStatement)
          .join("\n");
        for (const moduleName of modulesToInclude) {
          const body = moduleBodies.get(moduleName)!;
          outputText += `\nmod ${moduleName} {\n${body}\n} // ${moduleName}\n`;
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
