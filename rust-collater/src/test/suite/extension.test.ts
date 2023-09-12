// TODO: Simpler unit test framework that does not depend on VSCode.

import * as assert from "assert";
import * as vscode from "vscode";
import * as lib from "../../extensionLib";

function unindent(s: string): string {
  const INDENTATION_RE = /^ */;
  const lines = s.split("\n");
  const indents = lines
    .filter((line) => line.trim() !== "")
    .map((line) => line.match(INDENTATION_RE)![0].length);
  const minIndent = Math.min(...indents);
  return lines.map((line) => line.slice(minIndent)).join("\n");
}

suite("Extension Test Suite", () => {
  vscode.window.showInformationMessage("Start all tests.");

  test("Use statement simplification", () => {
    assert.deepEqual(
      lib.simplifyUseStatement("use std::collections::HashMap;"),
      ["use std::collections::HashMap;"]
    );
    assert.deepEqual(lib.simplifyUseStatement("use contest_lib_rs::foo;"), []);
    assert.deepEqual(
      lib.simplifyUseStatement("use contest_lib_rs::foo::bar;"),
      ["use foo::bar;"]
    );
    assert.deepEqual(lib.simplifyUseStatement("use contest_lib_rs::foo::*;"), [
      "use foo::*;",
    ]);
    assert.deepEqual(
      lib.simplifyUseStatement("use contest_lib_rs::foo::bar::buz;"),
      ["use foo::bar::buz;"]
    );
    assert.deepEqual(
      lib.simplifyUseStatement("use contest_lib_rs::{foo, bar};"),
      []
    );
    assert.deepEqual(
      lib.simplifyUseStatement("use contest_lib_rs::foo::{bar, baz};"),
      ["use foo::{bar, baz};"]
    );
    assert.deepEqual(
      lib.simplifyUseStatement("use contest_lib_rs::foo::bar::{baz, qux};"),
      ["use foo::bar::{baz, qux};"]
    );
    assert.deepEqual(
      lib.simplifyUseStatement("use contest_lib_rs::{foo, bar::baz};"),
      ["use bar::baz;"]
    );
    assert.deepEqual(
      lib.simplifyUseStatement(
        "use contest_lib_rs::{foo::bar, baz::qux::quux};"
      ),
      ["use {foo::bar, baz::qux::quux};"]
    );
    // TBD:
    // assert.deepEqual(
    //   ext.simplifyUseStatement("use contest_lib_rs::{foo, bar::{baz, qux}};"),
    //   ["use bar::{baz, qux};"]
    // );
  });

  test("Dependency detection", () => {
    const modFoo = unindent(`
        #[macro_export]
        macro_rules! my_macro {
            ( $x:expr ) => { $x };
        }
    `);
    const moduleTexts = new Map([
      ["foo", modFoo],
      ["bar", ""],
      ["baz", ""],
      ["qux", ""],
      ["quux", ""],
    ]);
    const modules = lib.parseModules(moduleTexts);

    const docIn = unindent(`\
        use contest_lib_rs::{my_macro, bar};
        use contest_lib_rs::baz::*;
        use contest_lib_rs::qux::{smth, smth_else};

        fn main() {}

    `);
    const modulesToInclude = lib.getModulesToInclude(docIn, modules);
    assert.deepEqual(modulesToInclude, new Set(["foo", "bar", "baz", "qux"]));
  });

  test("Collation", () => {
    const modIO = unindent(`
        #[macro_export]
        macro_rules! emitln {
            ( $dst:expr, $($value:expr),* ) => {{
                $crate::emit!($dst, $($value),*);
                writeln!($dst).unwrap();
            }};
        }
    `);
    const moduleTexts = new Map([
      ["graph", ""],
      ["io", modIO],
    ]);

    const docIn = unindent(`\
        use contest_lib_rs::{emitln, graph};

        fn main() {}
    `);
    const docOutExpected = unindent(`\
        fn main() {}

        mod graph {

        } // graph

        mod io {
        #[macro_export]
        macro_rules! emitln {
            ( $dst:expr, $($value:expr),* ) => {{
                $crate::emit!($dst, $($value),*);
                writeln!($dst).unwrap();
            }};
        }
        } // io
    `);
    const { outputText, missingModules } = lib.collateDocument(
      docIn,
      moduleTexts
    );
    assert.equal(outputText, docOutExpected);
    assert.deepEqual(missingModules, []);
  });

  test("Collation missing module", () => {
    const moduleTexts: Map<string, string> = new Map([]);

    const docIn = unindent(`\
        use contest_lib_rs::foo;

        fn main() {}
    `);
    const docOutExpected = unindent(`\
        fn main() {}

        mod foo {
        // NOT FOUND
        } // foo
    `);
    const { outputText, missingModules } = lib.collateDocument(
      docIn,
      moduleTexts
    );
    assert.equal(outputText, docOutExpected);
    assert.deepEqual(missingModules, ["foo"]);
  });

  test("Cargo modification", () => {
    const cargoIn = unindent(`\
        [workspace]
        members = [
            ".",
            "contests/round-1",
            "contests/round-2",
            "contests/round-7",
            "playground",
        ]

        [package]
        name = "contest_lib_rs"
        version = "0.1.0"
        edition = "2021"

        [dependencies]
    `);
    const cargoOutExpected = unindent(`\
        [workspace]
        members = [
            ".",
            "contests/round-1",
            "contests/round-2",
            "contests/round-3",
            "contests/round-7",
            "playground",
        ]

        [package]
        name = "contest_lib_rs"
        version = "0.1.0"
        edition = "2021"

        [dependencies]
    `);
    const cargoOutActual = lib.addCargoWorkspaceMember(
      cargoIn,
      "contests/round-3"
    );
    assert.equal(cargoOutActual, cargoOutExpected);
  });
});
