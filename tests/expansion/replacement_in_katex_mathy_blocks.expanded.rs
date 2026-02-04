use doxidize::doxidize;
/// Here's some comments that feel a bit like KaTex.
/// This calculates `$\log_c(\cos(x)+y)$`
/// which is completely useless, even more useless
/// than
///
/// ```
/// \begin{eqnarray}
/// p &=& \frac{x^y}{\sqrt{c}} \\
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
