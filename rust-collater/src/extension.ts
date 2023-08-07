// TODO: Rename the extension to reflect the fact that it does more than
// collation.

import { assert } from "console";
import { performance } from "perf_hooks";
import * as vscode from "vscode";

const CONTESTS_DIR = "contests";

async function fileExists(uri: vscode.Uri): Promise<boolean> {
  try {
    await vscode.workspace.fs.stat(uri);
    return true;
  } catch (e) {
    return false;
  }
}

async function getFileContent(uri: vscode.Uri): Promise<string> {
  const data = await vscode.workspace.fs.readFile(uri);
  return Buffer.from(data).toString("utf8");
}

async function setFileContent(uri: vscode.Uri, content: string) {
  await vscode.workspace.fs.writeFile(uri, Buffer.from(content, "utf8"));
}

async function guessWorkspaceRoot(): Promise<vscode.Uri | undefined> {
  const folders = vscode.workspace.workspaceFolders;
  if (!folders) {
    return undefined;
  }
  if (folders.length === 1) {
    return folders[0].uri;
  }
  for (const f of folders) {
    if (await fileExists(vscode.Uri.joinPath(f.uri, CONTESTS_DIR))) {
      return f.uri;
    }
  }
  return undefined;
}

function addCargoWorkspaceMember(
  cargoInput: string,
  member: string
): string | undefined {
  const SECTION_RE = /^\[(\w+)\]$/;
  let cargoLines = cargoInput.split("\n");
  let currentSection: string | undefined;
  let membersStart: number | undefined;
  let membersEnd: number | undefined;
  for (const [lineIdx, line] of cargoLines.entries()) {
    const lineTrimmed = line.trim();
    const sectionMatch = lineTrimmed.match(SECTION_RE);
    if (sectionMatch) {
      currentSection = sectionMatch[1];
    }
    if (currentSection === "workspace" && lineTrimmed === "members = [") {
      assert(membersStart === undefined);
      membersStart = lineIdx + 1;
    }
    if (
      membersStart !== undefined &&
      membersEnd === undefined &&
      lineTrimmed === "]"
    ) {
      membersEnd = lineIdx;
    }
  }
  if (membersStart === undefined || membersEnd === undefined) {
    return undefined;
  }
  const memberLines = cargoLines.slice(membersStart!, membersEnd!);
  memberLines.push(`    "${member}",`);
  memberLines.sort();
  cargoLines.splice(membersStart!, membersEnd! - membersStart!, ...memberLines);
  return cargoLines.join("\n");
}

// TODO: Support nested uses, e.g.
//   use contest_lib_rs::{io, emitln, graph::{VertexId, Graph}};
export function simplifyUseStatement(line: string): string[] {
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

  const SKIP_LINE_RE = /^(\/\/.*)?$/g;
  const TESTS_START_1 = "#[cfg(test)]";
  const TESTS_START_2 = "mod tests {";

  const RUST_SUFFIX = ".rs";

  context.subscriptions.push(
    vscode.commands.registerCommand("rust-collater.collate", async () => {
      const startTime = performance.now();

      const editor = vscode.window.activeTextEditor;
      if (!editor) {
        return;
      }

      try {
        // Skip `<contest_name>/src/<problem_name>.rs` and find the next Rust
        // folder.
        //
        // Improvement potential: Unify with `guessWorkspaceRoot` (but keep in
        // mind that `editor` should be taken into account here, but probably
        // not when creating a new contest).
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
          const moduleText = await getFileContent(
            vscode.Uri.joinPath(srcDir, fileName)
          );
          const moduleLines = moduleText.split("\n");
          let moduleBody = "";
          for (const [lineIdx, line] of moduleLines.entries()) {
            if (
              line.trim() === TESTS_START_1 &&
              moduleLines[lineIdx + 1].trim() === TESTS_START_2
            ) {
              break;
            }
            if (line.trim().match(SKIP_LINE_RE) === null) {
              moduleBody += line + "\n";
            }
          }
          moduleBodies.set(moduleName, moduleBody.trimEnd());
        }

        // Improvement potential: Cache macro mapping.
        // Improvement potential: Cache file content. (Check if VSCode does this
        // already.)
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
    })
  );

  // Improvement potential: Download tests from Codeforces.
  context.subscriptions.push(
    vscode.commands.registerCommand("rust-collater.new-contest", async () => {
      const rootDir = await guessWorkspaceRoot();
      if (rootDir === undefined) {
        vscode.window.showErrorMessage("Workspace root not found.");
        return;
      }
      const contestsRootDir = vscode.Uri.joinPath(rootDir, CONTESTS_DIR);
      const tmplDir = vscode.Uri.joinPath(contestsRootDir, "template");
      const cargoTmplPath = vscode.Uri.joinPath(tmplDir, "Cargo.toml.tmpl");
      const codeTmplPath = vscode.Uri.joinPath(tmplDir, "code.rs");
      const rootCargoPath = vscode.Uri.joinPath(rootDir, "Cargo.toml");
      let cargoTmpl, codeTmpl, rootCargo;
      try {
        cargoTmpl = await getFileContent(cargoTmplPath);
        codeTmpl = await getFileContent(codeTmplPath);
        rootCargo = await getFileContent(rootCargoPath);
      } catch (e: any) {
        vscode.window.showErrorMessage(e.toString());
        return;
      }

      const division = await vscode.window.showQuickPick([
        "Div. 1",
        "Div. 2",
        "Div. 3",
        "Div. 4",
        "Educational",
        // TODO: Add "Custom" option (e.g. for CodeTON).
      ]);
      if (division === undefined) {
        return;
      }
      const contestID = await vscode.window.showInputBox({
        placeHolder: "Contest ID",
      });
      if (contestID === undefined) {
        return;
      }

      const contestName =
        division === "Educational"
          ? `cf-educational-${contestID}`
          : `cf-round-${contestID}-${division.replace("Div. ", "div-")}`;
      const contestDir = vscode.Uri.joinPath(contestsRootDir, contestName);
      const contestSrcDir = vscode.Uri.joinPath(contestDir, "src");

      vscode.workspace.fs.createDirectory(contestDir);
      vscode.workspace.fs.createDirectory(contestSrcDir);
      const problemSet = ["a", "b", "c", "d", "e", "f"];
      for (const problem of problemSet) {
        setFileContent(
          vscode.Uri.joinPath(contestSrcDir, `${problem}.rs`),
          codeTmpl
        );
      }
      const cargoBinaries = problemSet.map((problem) => {
        return `[[bin]]\nname = "${problem}"\npath = "src/${problem}.rs"`;
      });
      setFileContent(
        vscode.Uri.joinPath(contestDir, "Cargo.toml"),
        cargoTmpl
          .replace("{{contest_name}}", contestName)
          .replace("{{binaries}}", cargoBinaries.join("\n\n"))
      );
      const rootCargoUpdated = addCargoWorkspaceMember(
        rootCargo,
        `${CONTESTS_DIR}/${contestName}`
      );
      if (rootCargoUpdated === undefined) {
        vscode.window.showErrorMessage("Failed to update root Cargo.toml.");
        return;
      }
      setFileContent(rootCargoPath, rootCargoUpdated);
      vscode.window.showInformationMessage(`Contest created: ${contestName}`);
    })
  );
}

// Called when the extension is deactivated.
export function deactivate() {}
