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
    assert.deepEqual(
      ext.simplifyUseStatement("use contest_lib_rs::{io, emitln};"),
      []
    );
    assert.deepEqual(
      ext.simplifyUseStatement("use contest_lib_rs::relax::RelaxMinMax;"),
      ["use relax::RelaxMinMax;"]
    );
  });
});
