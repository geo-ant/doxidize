# Doxidize - Documenting Function Parameters

![build](https://github.com/geo-ant/doxidize/actions/workflows/build.yml/badge.svg?branch=main)
![tests](https://github.com/geo-ant/doxidize/actions/workflows/tests.yml/badge.svg?branch=main)
![lints](https://github.com/geo-ant/doxidize/actions/workflows/lints.yml/badge.svg?branch=main)
[![crates](https://img.shields.io/crates/v/doxidize)](https://crates.io/crates/doxidize)
![maintenance-status](https://img.shields.io/badge/maintenance-actively--developed-brightgreen.svg)

The `#[doxidize]` attribute allows you to add doc-comments to function
parameters, which is a _compile error_ in current Rust. You can now write

```rust
use doxidize::*;

#[doxidize]
/// sum the rows of an image
fn sum_image_rows(
  /// the image data in row-major format
  image_data: &[f32],
  /// the number of rows in the image
  nrows: u32,
  /// the number of columns in the image
  ncols: u32,
  /// an out buffer into which the
  /// results of the sum of the rows are
  /// placed. Must have space for `nrows`
  /// elements
  sums: &mut [f32]) -> Result<(),String> {
    todo!()
} 
```

This will produce documentation as if you had written the doc-comment for the function
like so:

```rust
/// sum the rows of an image
///
/// **Arguments**: 
///
/// * `image_data`: the image data in row-major format
/// * `nrows`: the number of rows in the image
/// * `ncols`: the number of columns in the image
/// * `sums`:  an out buffer into which the
///    results of the sum of the rows are
///    placed. Must have space for `nrows`
///    elements
fn sum_image_rows(
  image_data: &[f32],
  nrows: u32,
  ncols: u32,
  sums: &mut [f32]) -> Result<(),String> {
    todo!()
}
```

## Placing the Arguments-Section

By default, the section documenting the arguments will go at the end
of the top-level function documentation. However, this crate allows to explicitly
place the section by using a custom attribute like so:

```rust
use doxidize::*;

#[doxidize]
/// long documention
/// ...
#[arguments_section]
/// # Examples
/// ...
fn foo(
  /// some docs
  first: i32,
  second: f32
  )
{}
```

## Considerations

It's a [long standing issue](https://github.com/rust-lang/rust/issues/57525)
whether and how to add this capability to `rustdoc`. Firstly, there's no
general consensus on how exactly to document function parameters. However, 
I've seen the presented style used a lot, with minor variations.
Secondly, the standard library [doesn't need this](https://github.com/rust-lang/rust/issues/57525#issuecomment-453633783)
style of documentation at all. So before you stick this macro on every function,
do consider

* taking inspiration from how the standard library deals with function parameters,
* using fewer function parameters,
* using more descriptive parameters names,
* using _types_ to communicate intent,
* sticking function parameters in a `struct`.

All that being said, I've realized that sometimes I still want to document
function parameters.

### Compile Times

Macros will always increase your compile time to some degree, but I don't think
this is a giant issue here for two reasons: firstly, this macro is to be used _sparingly_.
Secondly, this macro just does some light parsing and shuffling around of
the documentation tokens. It introduces no additional code. Thus, it doesn't
make your actual code more or less complex and should not affect compile
times much, but I haven't measured it... so take it with a grain of sodium-chloride.
