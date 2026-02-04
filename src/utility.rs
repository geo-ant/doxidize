use proc_macro2::TokenStream;
use quote::{ToTokens, TokenStreamExt};
use syn::{AttrStyle, Attribute, Signature, Visibility, parse::Parse};

/// This mirrors the ItemFn item in the syn crate with the difference
/// that the `block` isn't parsed, it's just a tokenstream, which should
/// be faster than parsing the actual block as well. We don't need the block
/// to be parsed for the logic inside this crate.
pub struct MinimalistItemFn {
    pub attrs: Vec<Attribute>,
    pub vis: Visibility,
    pub sig: Signature,
    pub block: TokenStream,
}

impl ToTokens for MinimalistItemFn {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        // NOTE(geo-ant): this is the same as the `ToTokens` implementation
        // for ItemFn in the syn crate. The only difference in implementation
        // is that the attributes are appended using
        //
        // ```
        // tokens.append_all(self.attrs.outer());
        // ```
        //
        // which I can't use here because it uses an internal crate iterator
        // helper, which does the same filtering as I do here.
        // Also, obviously the block tokenization is more involved in syn,
        // whereas we can just spit out the block as-is.

        tokens.append_all(
            self.attrs
                .iter()
                .filter(|attr| matches!(attr.style, AttrStyle::Outer)),
        );
        self.vis.to_tokens(tokens);
        self.sig.to_tokens(tokens);
        self.block.to_tokens(tokens);
    }
}

impl Parse for MinimalistItemFn {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        // same logic as in the syn crate with the difference that the
        // block is just read as is.
        Ok(Self {
            attrs: input.call(Attribute::parse_outer)?,
            vis: input.parse()?,
            sig: input.parse()?,
            block: input.parse()?,
        })
    }
}
