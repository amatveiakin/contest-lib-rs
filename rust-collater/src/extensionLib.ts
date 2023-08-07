import { assert } from "console";

// TODO: Support nested uses, e.g.
//   use contest_lib_rs::{io, emitln, graph::{VertexId, Graph}};
export function simplifyUseStatement(line: string): string[] {
  const INDENTATION_RE = /^(\s*)/;
  const SINGLE_USE_RE = /use contest_lib_rs::\w+;/;
  const NESTED_USE_RE = /use contest_lib_rs::(\w+::.*);/;
  const MULTI_USE_RE = /use contest_lib_rs::\{(.*)\};/;

  const trimmedLine = line.trim();
  const indentation = INDENTATION_RE.exec(line)![1];
  let uses = [];
  let match;

  if ((match = trimmedLine.match(SINGLE_USE_RE))) {
    // skip
  } else if ((match = trimmedLine.match(NESTED_USE_RE))) {
    uses.push(match[1]);
  } else if ((match = trimmedLine.match(MULTI_USE_RE))) {
    uses.push(
      ...match[1]
        .split(",")
        .map((s) => s.trim())
        .filter((s) => s.includes("::"))
    );
  } else {
    return [line];
  }

  if (uses.length === 0) {
    return [];
  } else if (uses.length === 1) {
    return [`${indentation}use ${uses[0]};`];
  } else {
    return [`${indentation}use {${uses.join(", ")}};`];
  }
}

export function moduleTextsToModuleBodies(
  moduleTexts: Map<string, string>
): Map<string, string> {
  const SKIP_LINE_RE = /^(\/\/.*)?$/g;
  const TESTS_START_1 = "#[cfg(test)]";
  const TESTS_START_2 = "mod tests {";

  const moduleBodies = new Map<string, string>();
  for (const [moduleName, moduleText] of moduleTexts.entries()) {
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
  return moduleBodies;
}

export function getModulesToInclude(
  currentText: string,
  moduleBodies: Map<string, string>
): Set<string> {
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
  return modulesToInclude;
}

export function collateDocument(
  currentText: string,
  moduleTexts: Map<string, string>
): string {
  const moduleBodies = moduleTextsToModuleBodies(moduleTexts);
  const modulesToInclude = getModulesToInclude(currentText, moduleBodies);

  let outputText = currentText
    .split("\n")
    .flatMap(simplifyUseStatement)
    .join("\n");
  outputText = outputText.trim() + "\n";
  for (const moduleName of modulesToInclude) {
    const body = moduleBodies.get(moduleName)!;
    outputText += `\nmod ${moduleName} {\n${body}\n} // ${moduleName}\n`;
  }
  return outputText;
}

export function addCargoWorkspaceMember(
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