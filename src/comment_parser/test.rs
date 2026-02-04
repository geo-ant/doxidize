use crate::comment_parser::{CommentLineToken, parse_line};
use proc_macro2::Span;

#[derive(Debug)]
enum CommentHelper<'a> {
    Verbatim(&'a str),
    Ident(&'a str),
}

impl<'a, 'b> PartialEq<CommentHelper<'a>> for CommentLineToken<'b> {
    fn eq(&self, other: &CommentHelper<'a>) -> bool {
        match other {
            CommentHelper::Verbatim(text) => {
                matches!(self, CommentLineToken::Verbatim(text2) if text2 == text)
            }
            CommentHelper::Ident(ident) => {
                matches!(self, CommentLineToken::Referred(referred) if &referred.ident == ident)
            }
        }
    }
}

#[test]
fn parse_weird_line() {
    let line = r#"p &=& \frac{@{x}^@{y}}{\sqrt{@{c}}} \\"#;
    let parsed = parse_line(line, Span::call_site());
    assert_eq!(parsed.len(), 7);
    assert_eq!(parsed[0], CommentHelper::Verbatim(r#"p &=& \frac{"#));
    assert_eq!(parsed[1], CommentHelper::Ident(r#"x"#));
    assert_eq!(parsed[2], CommentHelper::Verbatim(r#"^"#));
    assert_eq!(parsed[3], CommentHelper::Ident(r#"y"#));
    assert_eq!(parsed[4], CommentHelper::Verbatim(r#"}{\sqrt{"#));
    assert_eq!(parsed[5], CommentHelper::Ident(r#"c"#));
    assert_eq!(parsed[6], CommentHelper::Verbatim(r#"}} \\"#));
}
