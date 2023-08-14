// TODO: Simpler unit test framework that does not depend on VSCode.

import * as assert from "assert";
import * as vscode from "vscode";
import * as lib from "../../extensionLib";

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
});
