// TODO: Rename the extension to reflect the fact that it does more than
// collation.

import { performance } from "perf_hooks";
import * as vscode from "vscode";
import * as path from "path";
import { addCargoWorkspaceMember, collateDocument } from "./extensionLib";

const CONTESTS_DIR = "contests";
const SRC_DIR = "src";

const RUST_SUFFIX = ".rs";

const TMPL_DIR = "template";
const CARGO_TOML_TMPL = "Cargo.toml.tmpl";
const CODE_RS_TMPL = "code.rs";

const CONTEST_KINDS = [
  ["Codeforces Div. 1", "cf-round-{{id}}-div-1"],
  ["Codeforces Div. 2", "cf-round-{{id}}-div-2"],
  ["Codeforces Div. 3", "cf-round-{{id}}-div-3"],
  ["Codeforces Div. 4", "cf-round-{{id}}-div-4"],
  ["Codeforces Div. 1 + Div. 2", "cf-round-{{id}}-div-1-2"],
  ["Codeforces Educational", "cf-educational-{{id}}"],
  ["Codeforces Custom", "cf-{{kind}}-{{id}}"],
  ["Fully Custom", "{{full-name}}"],
];

const CONTEST_PLACEHOLDERS: {
  src: string;
  inputBoxOptions: vscode.InputBoxOptions;
}[] = [
  {
    src: "{{full-name}}",
    inputBoxOptions: {
      prompt: "Contest Folder Name, e.g. 'cf-round-123-div-1'",
    },
  },
  {
    src: "{{kind}}",
    inputBoxOptions: {
      prompt: "Contest Kind, e.g. 'codeton', 'april-fools'",
    },
  },
  {
    src: "{{id}}",
    inputBoxOptions: {
      prompt: "Contest ID",
    },
  },
];

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

async function commandCollate() {
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
    while (!(await fileExists(vscode.Uri.joinPath(rootDir, "Cargo.toml")))) {
      rootDir = vscode.Uri.joinPath(rootDir, "..");
    }
    const srcDir = vscode.Uri.joinPath(rootDir, "src");

    const moduleTexts = new Map<string, string>();
    const moduleFiles = await vscode.workspace.fs.readDirectory(srcDir);
    for (const [fileName, fileType] of moduleFiles) {
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
      moduleTexts.set(moduleName, moduleText);
    }

    const currentText = editor.document.getText();
    const { outputText, missingModules } = collateDocument(
      currentText,
      moduleTexts
    );

    vscode.env.clipboard.writeText(outputText.trimStart());

    const timeSpent = (performance.now() - startTime) / 1000.0;
    vscode.window.showInformationMessage(
      `Collated to clipboard (${timeSpent.toFixed(2)} s).`
    );
    if (missingModules.length > 0) {
      vscode.window.showWarningMessage(
        `The following modules were missing: ${missingModules.join(", ")}`
      );
    }
  } catch (e) {
    // Improvement potential: Better error messages:
    //   - for missing src directory;
    //   - for missing module (also highlight the `use` statement)
    vscode.window.showErrorMessage(`Collation error: ${e}`);
  }
}

async function updateContestCargoToml(
  contestName: string,
  contestDir: vscode.Uri,
  cargoTmpl: String
) {
  const srcDir = vscode.Uri.joinPath(contestDir, SRC_DIR);
  const problemSet = (await vscode.workspace.fs.readDirectory(srcDir))
    .filter(
      ([fileName, fileType]) =>
        fileType === vscode.FileType.File && fileName.endsWith(RUST_SUFFIX)
    )
    .map(([fileName, _]) => fileName.slice(0, -RUST_SUFFIX.length));
  const cargoBinaries = problemSet.map((problem) => {
    return `[[bin]]\nname = "${problem}"\npath = "${SRC_DIR}/${problem}${RUST_SUFFIX}"`;
  });
  setFileContent(
    vscode.Uri.joinPath(contestDir, "Cargo.toml"),
    cargoTmpl
      .replace("{{contest_name}}", contestName)
      .replace("{{binaries}}", cargoBinaries.join("\n\n"))
  );
}

// Improvement potential: Download tests from Codeforces. In order to speed
// things up, this should happen after the contest is created. This would allow
// Rust Language Server to warm up, and also it means one could begin working on
// the first problem without waiting the reply from Codeforces, which is often
// slow in the first moments of the contest.
// Solution idea. Create an empty contest consisting of valid Rust source code
// files with empty tests, like we do now. Leave
// ```
//   // {{codeforces-tests}}
// ```
// comment in each file. Replace the comments with the actual tests later while
// preserving changes to other parts of the file.
async function commandNewContest() {
  const rootDir = await guessWorkspaceRoot();
  if (rootDir === undefined) {
    vscode.window.showErrorMessage("Workspace root not found.");
    return;
  }
  const contestsRootDir = vscode.Uri.joinPath(rootDir, CONTESTS_DIR);
  const tmplDir = vscode.Uri.joinPath(contestsRootDir, TMPL_DIR);
  const cargoTmplPath = vscode.Uri.joinPath(tmplDir, CARGO_TOML_TMPL);
  const codeTmplPath = vscode.Uri.joinPath(tmplDir, CODE_RS_TMPL);
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

  const kind = await vscode.window.showQuickPick(
    CONTEST_KINDS.map(([label, _]) => label)
  );
  if (kind === undefined) {
    return;
  }
  let contestName = CONTEST_KINDS.find(([label, _]) => label === kind)![1];

  for (const { src, inputBoxOptions } of CONTEST_PLACEHOLDERS) {
    if (contestName.includes(src)) {
      const value = await vscode.window.showInputBox(inputBoxOptions);
      if (value === undefined) {
        return;
      }
      contestName = contestName.replace(src, value);
    }
  }

  const contestDir = vscode.Uri.joinPath(contestsRootDir, contestName);
  const contestSrcDir = vscode.Uri.joinPath(contestDir, "src");

  vscode.workspace.fs.createDirectory(contestDir);
  vscode.workspace.fs.createDirectory(contestSrcDir);
  const problemSet = ["a", "b", "c", "d", "e", "f"];
  for (const problem of problemSet) {
    setFileContent(
      vscode.Uri.joinPath(contestSrcDir, `${problem}${RUST_SUFFIX}`),
      codeTmpl
    );
  }

  await updateContestCargoToml(contestName, contestDir, cargoTmpl);
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
}

async function commandRemoveEmptySolutions() {
  const rootDir = await guessWorkspaceRoot();
  if (rootDir === undefined) {
    vscode.window.showErrorMessage("Workspace root not found.");
    return;
  }
  const contestsRootDir = vscode.Uri.joinPath(rootDir, CONTESTS_DIR);
  const tmplDir = vscode.Uri.joinPath(contestsRootDir, TMPL_DIR);
  const cargoTmplPath = vscode.Uri.joinPath(tmplDir, CARGO_TOML_TMPL);
  const codeTmplPath = vscode.Uri.joinPath(tmplDir, CODE_RS_TMPL);
  let cargoTmpl, codeTmpl;
  try {
    cargoTmpl = await getFileContent(cargoTmplPath);
    codeTmpl = await getFileContent(codeTmplPath);
  } catch (e: any) {
    vscode.window.showErrorMessage(e.toString());
    return;
  }

  for (const [
    contestName,
    contestFileType,
  ] of await vscode.workspace.fs.readDirectory(contestsRootDir)) {
    if (contestFileType !== vscode.FileType.Directory) {
      continue;
    }
    if (contestName === TMPL_DIR) {
      continue;
    }
    const contestDir = vscode.Uri.joinPath(contestsRootDir, contestName);
    const srcDir = vscode.Uri.joinPath(contestDir, SRC_DIR);
    for (const [
      solutionName,
      solutionFileType,
    ] of await vscode.workspace.fs.readDirectory(srcDir)) {
      if (solutionFileType !== vscode.FileType.File) {
        continue;
      }
      if (!solutionName.endsWith(RUST_SUFFIX)) {
        continue;
      }
      const solutionPath = vscode.Uri.joinPath(srcDir, solutionName);
      const solutionText = await getFileContent(solutionPath);
      // TODO: A fuzzy detector for "basically empty" solutions.
      if (solutionText === codeTmpl) {
        await vscode.workspace.fs.delete(solutionPath);
      }
    }
    await updateContestCargoToml(contestName, contestDir, cargoTmpl);
  }
}

// Called the very first time a command is executed.
export function activate(context: vscode.ExtensionContext) {
  context.subscriptions.push(
    vscode.commands.registerCommand("rust-collater.collate", commandCollate)
  );

  context.subscriptions.push(
    vscode.commands.registerCommand(
      "rust-collater.new-contest",
      commandNewContest
    )
  );

  context.subscriptions.push(
    vscode.commands.registerCommand(
      "rust-collater.remove-empty-solutions",
      commandRemoveEmptySolutions
    )
  );
}

// Called when the extension is deactivated.
export function deactivate() {}
