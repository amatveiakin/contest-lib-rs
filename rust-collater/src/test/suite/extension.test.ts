import * as assert from "assert";
import * as vscode from "vscode";
import * as ext from "../../extension";

suite("Extension Test Suite", () => {
  vscode.window.showInformationMessage("Start all tests.");

  test("Use statement simplification", () => {
    assert.deepEqual(
      ext.simplifyUseStatement("use std::collections::HashMap;"),
      ["use std::collections::HashMap;"]
    );
    assert.deepEqual(ext.simplifyUseStatement("use contest_lib_rs::foo;"), []);
    assert.deepEqual(
      ext.simplifyUseStatement("use contest_lib_rs::foo::bar;"),
      ["use foo::bar;"]
    );
    assert.deepEqual(
      ext.simplifyUseStatement("use contest_lib_rs::foo::bar::buz;"),
      ["use foo::bar::buz;"]
    );
    assert.deepEqual(
      ext.simplifyUseStatement("use contest_lib_rs::{foo, bar};"),
      []
    );
    assert.deepEqual(
      ext.simplifyUseStatement("use contest_lib_rs::foo::{bar, baz};"),
      ["use foo::{bar, baz};"]
    );
    assert.deepEqual(
      ext.simplifyUseStatement("use contest_lib_rs::foo::bar::{baz, qux};"),
      ["use foo::bar::{baz, qux};"]
    );
    assert.deepEqual(
      ext.simplifyUseStatement("use contest_lib_rs::{foo, bar::baz};"),
      ["use bar::baz;"]
    );
    assert.deepEqual(
      ext.simplifyUseStatement(
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
