# doxidize - fearless function documentation 

The `#[doxidize]` attribute allows us to document functions in a refactoring-proof
manner.

## Refactoring-proof Your Documentation

If you've ever had the problem of function documentation going out of sync
with your signature, this macro is for you.  Stick the `#[doxidize]` 
attribute on top of your function documentation and
start referring to any parameter `param` using `@{param}` in the comments. This
also works for generics in the signature.

```rust
use doxidize::doxidize;

#[doxidize]
/// Sums the rows of an image.
///
/// The rows of `@{image_data}`, an `@{nrows}` by `@{ncols}`
/// matrix in row-major ordering, are summed into `$sums`
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
with backticks to make it part of the regular text:

```rust
use doxidize::doxidize;

#[doxidize]
/// Apply a blur to an image given @{width} and @{height}, where
/// the corresponding data is `@{image_data}`.
fn blur(
  image_data: &mut [f32], width: u32, height: u32) {
    todo!()
}
```

This creates naturally sounding docs that are kept in sync with your actual
parameter names. Every `@{param}` will be checked and, if valid, be replaced
with `param`, even inside code blocks (which should be an unlikely use case).
If you ever need to write `@{param}` without it being replaced, just escape
it by using `@@{param}`.

## Recommended Usage

I suggest to use this macro conditionally like so:

```rust
#[cfg_attr(doc,doxidize)]
/// The argument `$arg` is a dummy.
fn foo(arg: i32) {}
```

This will trigger the macro only if you invoke `cargo test` or `cargo doc`, 
not on a regular `build` or `check` command. This might be less annoying
when working in your editor of choice.

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
