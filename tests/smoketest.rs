use doxidize::*;

#[doxidize]
/// The parameter `$first` is kept in sync, but there is no parameter `bar`.
/// There is a `$second` parameter though and a generic argument called `$T`
/// generic and a const generic `$C`.
fn foo<T, U, const C: i32>(first: i32, second: f32) -> f32 {
    first as f32 - second
}

#[test]
fn test_foo() {
    assert_eq!(foo::<i32, (), 4>(1, 3.), -2.);
}
