# doxidize - fearless function documentation 

![build](https://github.com/geo-ant/doxidize/actions/workflows/build.yml/badge.svg?branch=main)
![tests](https://github.com/geo-ant/doxidize/actions/workflows/tests.yml/badge.svg?branch=main)
![lints](https://github.com/geo-ant/doxidize/actions/workflows/lints.yml/badge.svg?branch=main)
![Crates.io Version](https://img.shields.io/crates/v/doxidize)
[![support](https://raw.githubusercontent.com/geo-ant/user-content/refs/heads/main/ko-fi-support.svg)](https://ko-fi.com/geoant)

**Refactoring-proof your function docs**

## Usage

If you've ever had the problem of function documentation going out of sync
with function signatures, this macro is for you. Stick the `#[doxidize]` 
attribute on top of your function documentation and
start referring to any parameter `param` using `@{param}` in the comments to enforce
that it actually exists. This also works for generics in the signature.

```rust
use doxidize::doxidize;

#[doxidize]
/// Sums the rows of an image.
///
/// The rows of `@{image_data}`, an `@{nrows}` by `@{ncols}`
/// matrix in row-major ordering, are summed into `@{sums}`
/// which must have exactly `@{nrows}` elements.
fn sum_image_rows(image_data: &[f32],
                  nrows: u32,
                  ncols: u32,
                  sums: &mut [f32]) -> Result<(),String> {
    todo!()
}
```

This will create your function documentation as if you had just written `param`
instead of `@{param}`, but will make it a **compile-time error** to refer to a non-existent
parameter or generic. You can also use `@{param}` without surrounding it
with backticks to make it part of the natural text:

```rust
use doxidize::doxidize;

#[doxidize]
/// Apply a blur to an image of known @{width} and @{height},
/// where the raw image data resides in `@{image_data}`
/// in a row-major fashion.
fn blur(image_data: &mut [f32], width: u32, height: u32) {
    todo!()
}
```

This creates naturally sounding docs that are kept in sync with your actual
parameter names. Every `@{param}` will be checked and, if valid, be replaced
with `param`, even inside code blocks (which should be an unlikely use case).
If you ever need to write `@{param}` without it being replaced, just escape
it by using `@@{param}`.

## Context, Alternatives, Acknowledgements

Function parameter documentation in Rust is a 
[long standing and contentious](https://github.com/rust-lang/rust/issues/57525) 
issue. I'm also the author of the [`roxygen`](https://docs.rs/roxygen/latest/roxygen/)
crate that tackles this problem by allowing in-code documentation of the parameters,
which comes with its own set of tradeoffs, some of which apply here as well. 

The idea for the `doxidize` crate comes from a 
[comment](https://github.com/rust-lang/rust/issues/57525#issuecomment-3768426722)
of GitHub user [`blueglyph`](https://github.com/blueglyph) in the issue linked
above. See this crate as a polyfill until `rustdoc` settles on an official
way... which might take a while.
