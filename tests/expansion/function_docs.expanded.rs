tokenized: [Verbatim(" this is documentation")]
tokenized: [Referred(ReferredIdent { ident: "S", span: #0 bytes(56..126) }), Verbatim("` but not `T`.")]
tokenized: [Verbatim(" Also something about `x = 4` and `y`, which don't exit.")]
tokenized: [Verbatim(" Some more comments.")]
tokenized: [Referred(ReferredIdent { ident: "N", span: #0 bytes(211..263) }), Referred(ReferredIdent { ident: "bar", span: #0 bytes(211..263) }), Referred(ReferredIdent { ident: "baz", span: #0 bytes(211..263) }), Verbatim("`.")]
#![feature(prelude_import)]
#[macro_use]
extern crate std;
#[prelude_import]
use std::prelude::rust_2024::*;
use doxidize::*;
/// this is documentation
///S` but not `T`.
/// Also something about `x = 4` and `y`, which don't exit.
/// Some more comments.
///Nbarbaz`.
fn foo<
    /// a lifetime
    'a,
    S,
    /// documentation for parameter T
    /// spans multiple lines
    T,
    /// a const generic
    const N: usize,
>(
    /// this has one line of docs
    bar: u32,
    /// this has
    /// two lines of docs
    baz: String,
    _undocumented: i32,
) -> bool {
    if bar < 100 { baz.len() > bar as usize } else { false }
}
