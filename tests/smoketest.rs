#![allow(dead_code)]
use doxidize::*;

#[doxidize]
/// The parameter `@{first}` is kept in sync, but there is no parameter `bar`.
/// There is a `@{second}` parameter though and a generic argument called `@{T}`
/// generic and a const generic `@{C}`.
fn foo<T, U, const C: i32>(first: i32, second: f32) -> f32 {
    first as f32 - second
}

#[doxidize]
/// Let `$@{x}$` or `$$@{x}$$` be katex comments and `@{x}` and `@{y}`
/// refer to the parameters but `$y$` and `$x+y$` do not.
/// This should leave the katex stuff untouched...
fn mathy_fun(x: i32, y: i32) -> i32 {
    x * y + y
}

#[doxidize]
/// Here's some comments that feel a bit like KaTex.
/// This calculates `$\log_@{c}(\cos(@{x})+@{y})$`
/// which is completely useless, even more useless
/// than
///
/// ```
/// \begin{eqnarray}
/// p &=& \frac{@{x}^@{y}}{\sqrt{@{c}}} \\
/// q &=& 4
/// \end{eqnarray}
/// ```
///
/// which is also pretty useless!
fn calc(x: f32, y: f32, c: f32) -> f32 {
    let cx = x.cos();
    let cxpy = cx + y;
    (cxpy).log(c)
}
