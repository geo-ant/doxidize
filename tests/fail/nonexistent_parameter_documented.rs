use doxidize::*;

#[doxidize]
/// let's try to document a function parameter `$foo` that exists,
/// but also `$bar` which does not, but the generic `$T` exists.
fn func<T>(foo: i32) {}

fn main() {}
