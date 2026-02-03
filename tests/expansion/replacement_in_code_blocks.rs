use doxidize::doxidize;

#[doxidize]
/// @{foo} and ``@{bar}`` are parameters.
///
/// This is a code block
/// ```rust
///    // why would you do that???
///    let @{bar} = 10;
///    let result = funky(@{bar},10);
///    // some escaping niche cases
///    let domain = "google.com";
///    println!("someone@@{domain}.com");
/// ```
#[allow(dead_code)]
fn add(foo: i32, bar: i32) -> i32 {
    foo + bar
}
