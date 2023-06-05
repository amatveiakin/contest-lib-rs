# rust-collater README

Combines multiple Rust modules into a single file for online judgment systems.

Keeps the main file intact (expect for the `use` statements), while compressing
the auxiliary modules:

* Strips away comments;
* (TBD) Strips away tests;

## Requirements

None (in particular, Rust compiler and RLS are not required).

## Extension Settings

None.
