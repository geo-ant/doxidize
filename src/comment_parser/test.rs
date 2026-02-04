use proc_macro2::Span;

use crate::comment_parser::parse_line;

#[test]
fn parse_weird_line() {
    let line = r#"p &=& \frac{@{x}^@{y}}{\sqrt{@{c}}} \\"#;
    let parsed = parse_line(line, Span::call_site());
}
