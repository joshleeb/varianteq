use quote::{ToTokens, Tokens};
use std::iter::repeat;
use syn::{Fields, Ident, Variant};

pub struct EnumVariant<'a> {
    enum_ident: &'a Ident,
    variant: Variant,
}

impl<'a> EnumVariant<'a> {
    pub fn new(enum_ident: &'a Ident, variant: Variant) -> Self {
        EnumVariant {
            enum_ident,
            variant,
        }
    }
}

impl<'a> ToTokens for EnumVariant<'a> {
    fn to_tokens(&self, tokens: &mut Tokens) {
        let enum_ident = self.enum_ident;
        let variant_ident = self.variant.ident;

        let match_pattern = match self.variant.fields {
            Fields::Unit => quote!(#enum_ident::#variant_ident),
            Fields::Named(_) => quote!(#enum_ident::#variant_ident{..}),
            Fields::Unnamed(ref field) => {
                let underscores = repeat(quote!(_)).take(field.unnamed.len());
                quote!(#enum_ident::#variant_ident(#(#underscores),*))
            }
        };
        tokens.append_all(quote!((#match_pattern, #match_pattern)));
    }
}
