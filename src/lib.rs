#![doc= include_str!("../Readme.md")]

use proc_macro2::Span;
use quote::{ToTokens, quote};
use std::collections::HashSet;
use syn::{Expr, FnArg, ItemFn, Meta, MetaNameValue, Pat, parse_macro_input, spanned::Spanned};

use crate::comment_parser::parse_coments;

// from today's clippy threshold for the "too many arguments" lint, so should
// be useful for constructing our internal buffer size
const REASONABLE_MAX_NUMBER_OF_FUNCTION_PARAMS: usize = 7;
const REASONABLE_MAX_NUMBER_OF_FUNCTION_GENERICS: usize = 7;

struct DocCommentLine {
    comment: String,
    span: Span,
}

mod comment_parser;

#[proc_macro_attribute]
/// The principal macro attribute in this crate that lets us keep function
/// documentation in sync with the function signature.
pub fn doxidize(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    // TODO(geo-ant): this parses also the function body, which is definitely
    // overkill. I should come up with a way, to parse only the attributes
    // and the signature. This can probably be done by extracting the relevant
    // code from the syn crate and hacking it a bit.
    let mut function: ItemFn = parse_macro_input!(item as ItemFn);

    // this now constains the whole list of parameter names, generic param
    // names and const generic names.
    let generics_and_params_names = extract_function_parameter_and_generics_indentifiers(&function);

    if generics_and_params_names.is_empty() {
        return syn::Error::new(
            function.sig.span(),
            "Function has no parameters or generics to document. Remove the #[doxidize] attribute.",
        )
        .to_compile_error()
        .to_token_stream()
        .into();
    }

    // we now remove all the doc strings from the function and collect
    // all the individual lines here.
    let doc_string_lines: Vec<DocCommentLine> = function
        .attrs
        // this removes all the doc attributes from the original vector
        .extract_if(.., |attr| attr.path().is_ident("doc"))
        // we now know this is a #[doc = "..."] attribute so we can extract
        // the string from this
        .map(|attr| {
            if let Meta::NameValue(MetaNameValue {
                value: Expr::Lit(ref lit),
                ..
            }) = attr.meta
            {
                let syn::ExprLit {
                    attrs: _,
                    lit: syn::Lit::Str(doc_string),
                } = lit
                else {
                    unreachable!("reached unexpected node while parsing");
                };

                let comment = doc_string.value();
                let span = lit.span();
                DocCommentLine { comment, span }
            } else {
                unreachable!("reached unexpected node while parsing");
            }
        })
        .collect();

    let parsed = parse_coments(&doc_string_lines);

    let mut is_any_parameter_documented = false;
    // just very simple parsing which just searches for the backticks and
    // check the stuff inside the ticks against the allowed generic, const generic,
    // and parameter names if it begins with a marker.
    for param in parsed.referred() {
        is_any_parameter_documented = true;
        if !generics_and_params_names.contains(param.ident) {
            return syn::Error::new(
                param.span,
                format!(
                    "documented item '{}' is not part of the function signature!",
                    &param.ident
                ),
            )
            .to_compile_error()
            .to_token_stream()
            .into();
        }
    }

    if !is_any_parameter_documented {
        return syn::Error::new(
            function.sig.span(),
            format!("No parameters documented!\nUse the `$identifier` syntax (e.g. `${}`) to refer to a function parameter or generic or consider removing the `#[doxidize]` attribute.", generics_and_params_names.iter().next().unwrap()),
        )
        .to_compile_error()
        .to_token_stream()
        .into();
    }

    let new_doc_strings = parsed.stringify_lines().map(|comment| {
        quote! {
            #[doc = #comment]
        }
    });

    quote! {
        #(#new_doc_strings)*
        #function
    }
    .into()
}

/// extract the parameter names (identifiers) from a function
fn extract_function_parameter_and_generics_indentifiers(function: &ItemFn) -> HashSet<String> {
    let parameters = function.sig.inputs.iter();
    let (params_lower, params_upper) = parameters.size_hint();

    let generics = function.sig.generics.params.iter();
    let (generics_lower, generics_upper) = generics.size_hint();

    let capacity = params_upper
        .unwrap_or(REASONABLE_MAX_NUMBER_OF_FUNCTION_PARAMS)
        .max(params_lower)
        + generics_upper
            .unwrap_or(REASONABLE_MAX_NUMBER_OF_FUNCTION_GENERICS)
            .max(generics_lower);

    let mut idents = HashSet::with_capacity(capacity);

    for arg in parameters {
        match arg {
            FnArg::Typed(pat_type) => {
                let Pat::Ident(pat_ident) = pat_type.pat.as_ref() else {
                    unreachable!("unexpected node while parsing");
                };
                let ident = &pat_ident.ident;
                idents.insert(ident.to_string());
            }
            FnArg::Receiver(_) => {}
        }
    }

    for gen_arg in generics {
        match gen_arg {
            // no docs for lifetime params allowed. I think those are basically
            // self documenting through the bounds, but I might be convinced otherwise
            syn::GenericParam::Lifetime(_) => {}
            syn::GenericParam::Type(type_param) => {
                idents.insert(type_param.ident.to_string());
            }
            syn::GenericParam::Const(const_param) => {
                idents.insert(const_param.ident.to_string());
            }
        }
    }
    idents
}
