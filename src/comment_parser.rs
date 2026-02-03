use std::char::ParseCharError;

use crate::{DocCommentLine, REASONABLE_MAX_NUMBER_OF_FUNCTION_PARAMS};
use proc_macro::TokenStream;
use proc_macro2::Span;

const ESCAPE_PREFIX: &str = "@";
const OPEN_MARKER: &str = "@{";
const CLOSE_MARKER: char = '}';

#[derive(Debug, Clone)]
pub struct ParsedComment<'a> {
    parsed_lines: Vec<Vec<CommentLineToken<'a>>>,
}

pub fn parse_coments<'a>(lines: &'a [DocCommentLine]) -> ParsedComment<'a> {
    let parsed_lines = lines
        .into_iter()
        .map(|line| parse_line(&line.comment, line.span))
        .collect();

    ParsedComment { parsed_lines }
}

impl<'a> ParsedComment<'a> {
    /// strongly typed iterators over all the referred identifiers.
    pub fn referred(&self) -> impl Iterator<Item = ReferredIdent> {
        // jeez, I just love rust iterators... try doing this shit in C++
        self.parsed_lines.iter().flat_map(|line| {
            line.iter().flat_map(|token| match token {
                CommentLineToken::Referred(referred) => Some(*referred),
                _ => None,
            })
        })
    }

    /// transform all lines into strings the way we want to output them
    /// (with the @{param} replaced by param)
    pub fn stringify_lines(&self) -> impl ExactSizeIterator<Item = String> {
        self.parsed_lines.iter().map(|tokens| stringify(tokens))
    }
}

fn stringify<'a>(tokens: &[CommentLineToken<'a>]) -> String {
    let total_len = tokens.iter().map(CommentLineToken::len).sum();
    let mut out = String::with_capacity(total_len);

    for token in tokens {
        out.push_str(token.as_str());
    }
    out
}

#[derive(Debug, Copy, Clone)]
pub struct ReferredIdent<'a> {
    pub ident: &'a str,
    pub span: Span,
}

#[derive(Debug, Copy, Clone)]
/// this is a bit of a hacky mixture between tokenization and actual parsing,
/// I think. The way I set up the "grammar" for referring to the parameters,
/// I can make a pretty fast, reasonably robust, and simple parser that still
/// allows for pretty powerful subsitutions.
enum CommentLineToken<'a> {
    /// this text just needs to be copied verbatim to the output comment
    Verbatim(&'a str),
    /// a referred identifier (anything between @{...}) without the leading
    /// '@{' and closing '}'. This identifier can be checked if it exists
    /// in the function signature. If printed, will just print the '...'
    // NOTE(geo) the only restriction we have for now is that @{...} needs
    // to exist on one line, which really shouldn't be a limitation in practice.
    Referred(ReferredIdent<'a>),
    /// our way to escape the @{...} syntax is to write @@{...}. In this case
    /// what is literally printed is '@{...}', without the first leading
    /// '@'
    Escaped(&'a str),
}

impl<'a> CommentLineToken<'a> {
    const fn referred(ident: &'a str, span: Span) -> Self {
        Self::Referred(ReferredIdent { ident, span })
    }

    const fn empty() -> Self {
        Self::Verbatim("")
    }

    /// the len of the comment token if it was printed into a string
    const fn len(&self) -> usize {
        match self {
            CommentLineToken::Verbatim(text) => text.len(),
            CommentLineToken::Referred(referred) => referred.ident.len(),
            CommentLineToken::Escaped(text) => text.len(),
        }
    }

    const fn as_str(&'a self) -> &'a str {
        match self {
            CommentLineToken::Verbatim(text) => text,
            CommentLineToken::Referred(referred_ident) => referred_ident.ident,
            CommentLineToken::Escaped(text) => text,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Error;

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error")
    }
}

impl std::error::Error for Error {}

enum LineState {
    /// start a fresh line
    Init,
    BlockComment {
        fence_len: usize,
    },
    InlineCode {
        fence_len: usize,
    },
}

pub fn lex_markdown<'a, L: ExactSizeIterator<Item = (&'a str, proc_macro2::Span)>>(
    lines: L,
) -> Result<Vec<CommentLineToken<'a>>, syn::Error> {
    let mut state = LineState::Init;
    // we expect less than 80-is characters per line, so actually way less than
    // 80 tokens. But the token enum is small enough to not be wasteful if
    // we go 80.
    let mut lexed = Vec::with_capacity(lines.len() * 80);

    for (line, span) in lines {
        let mut read = &line[0..0];
        let mut start = 0;
        let mut line_tokens = Vec::<CommentLineToken>::with_capacity(80);
        for (index, character) in line.chars().enumerate() {
            //     match state {
            //         LexerState::LineInit => match character {
            //             MARKER => {},
            //             c if c.is_whitespace() {
            //             [

            //         }
            //             _ => {
            //                 read = &line[start..index + 1];
            //             }
            //         },
            //         LexerState::BlockComment { fence_len } => todo!(),
            //         LexerState::InlineCode { fence_len } => todo!(),
            //     }
        }
    }

    Ok(lexed)
}

fn parse_line(line: &'_ str, span: Span) -> Vec<CommentLineToken<'_>> {
    // special casing empty lines because otherwise it's too easy to remove them
    if line.is_empty() {
        return vec![CommentLineToken::empty()];
    }

    let mut tokenized = Vec::with_capacity(3 * REASONABLE_MAX_NUMBER_OF_FUNCTION_PARAMS);
    let mut start_index = 0;

    while start_index < line.len() {
        let Some(open_marker_pos) = (&line[start_index..])
            .find(OPEN_MARKER)
            .map(|pos| pos + start_index)
        else {
            // if there's no open marker, the whole line is just on big
            // verbatim token.
            tokenized.push(CommentLineToken::Verbatim(&line[start_index..]));
            break;
        };

        let Some(closed_marker_pos) = (&line[start_index..])
            .find(CLOSE_MARKER)
            .map(|pos| pos + start_index)
        else {
            tokenized.push(CommentLineToken::Verbatim(&line[start_index..]));
            break;
        };

        // since we can't index into a string to get the previous character we
        // use the slightly hacky variant of creating a substring
        if open_marker_pos >= ESCAPE_PREFIX.len()
            || &line[open_marker_pos - ESCAPE_PREFIX.len()..open_marker_pos] == ESCAPE_PREFIX
        {
            // this is the escaped case: we just leave out the escaping character
            // and copy the rest into the token, which is for @@{...} would
            // be @{...}
            tokenized.push(CommentLineToken::Escaped(
                &line[open_marker_pos..closed_marker_pos + 1],
            ));
            start_index = closed_marker_pos + 1;
        } else {
            // this is the case where the user marked @{param} without escape
            // and we just want to parse "param" as the referred ident
            tokenized.push(CommentLineToken::referred(
                &line[open_marker_pos + 1..closed_marker_pos],
                span,
            ));
        }
    }
    return tokenized;
}
