import { assert } from "console";

interface RustModule {
  body: string;
  macros: string[];
  dependencies: string[];
}

interface CollationResult {
  outputText: string;
  missingModules: string[];
}

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

export function stripTests(moduleText: string): string {
  const TESTS_START_1 = "#[cfg(test)]";
  const TESTS_START_2 = "mod tests {";
  const moduleLines = moduleText.split("\n");
  const startIdx = moduleLines.findIndex(
    (line) => line.trim() === TESTS_START_1
  );
  if (startIdx >= 0 && moduleLines[startIdx + 1].trim() === TESTS_START_2) {
    moduleLines.splice(startIdx);
  }
  return moduleLines.join("\n");
}

// TODO: Also strip comments on code lines.
export function stripEmptyLinesAndComments(moduleText: string): string {
  const SKIP_LINE_RE = /^(\/\/.*)?$/g;
  return moduleText
    .split("\n")
    .filter((line) => line.trim().match(SKIP_LINE_RE) === null)
    .join("\n");
}

export function parseModule(moduleText: string): RustModule {
  const MODULE_DEPENDENCY_RE = [
    /use crate::\{(\w+)(?:::\w+)*(?:, *(\w+)(?:::\w+)*)*\}/g,
    /use crate::(\w+)/g,
  ];
  const MACRO_RE = /macro_rules! +(\w+)/g;

  let body = stripEmptyLinesAndComments(stripTests(moduleText));

  const macros: string[] = [];
  for (const match of body.matchAll(MACRO_RE)) {
    macros.push(match[1]);
  }
  const dependencies: string[] = [];
  for (const re of MODULE_DEPENDENCY_RE) {
    for (const match of body.matchAll(re)) {
      dependencies.push(...match.slice(1));
    }
  }

  return {
    body: body,
    macros: macros,
    dependencies: dependencies,
  };
}

export function parseModules(
  moduleTexts: Map<string, string>
): Map<string, RustModule> {
  return new Map(
    Array.from(moduleTexts.entries()).map(([k, v]) => [k, parseModule(v)])
  );
}

export function getModulesToInclude(
  currentText: string,
  modules: Map<string, RustModule>
): Set<string> {
  // TODO: Support multiline use statements and macro definitions.
  // Better yet: proper Rust parser.
  const USE_MODULE_RE = [
    /use contest_lib_rs::\{(\w+)(?:::\w+)*(?:, *(\w+)(?:::\w+)*)*\}/g,
    /use contest_lib_rs::(\w+)/g,
  ];

  const macroDefinitions = new Map<string, string>();
  for (const [moduleName, module] of modules.entries()) {
    for (const macro of module.macros) {
      macroDefinitions.set(macro, moduleName);
    }
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
      dependencies.push(...(modules.get(module)?.dependencies || []));
    }
  }
  return modulesToInclude;
}

export function collateDocument(
  currentText: string,
  moduleTexts: Map<string, string>
): CollationResult {
  const modules = parseModules(moduleTexts);
  const modulesToInclude = getModulesToInclude(
    stripTests(currentText),
    modules
  );

  let outputText = currentText
    .split("\n")
    .flatMap(simplifyUseStatement)
    .join("\n");
  outputText = outputText.trim() + "\n";
  let missingModules: string[] = [];
  for (const moduleName of modulesToInclude) {
    let body = modules.get(moduleName)?.body;
    if (body === undefined) {
      missingModules.push(moduleName);
      body = "// NOT FOUND";
    }
    outputText += `\nmod ${moduleName} {\n${body}\n} // ${moduleName}\n`;
  }
  return { outputText, missingModules };
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
