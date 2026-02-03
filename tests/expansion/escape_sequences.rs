use doxidize::*;

#[doxidize]
/// @{bar} with more weird docs. We're documenting the generic `@{S}` and we're
/// documenting @{T} without backticks.
/// Also something about `x = 4` and `y`, which don't exit.
/// Some more comments.
/// But then there's `@{N}`, let's escape @@{bar}, and document `@{baz}`.
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
    if bar < 100 {
        baz.len() > bar as usize
    } else {
        false
    }
}
