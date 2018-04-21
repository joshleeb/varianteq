use quote::{ToTokens, Tokens};
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

        match self.variant.fields {
            Fields::Unit => tokens.append_all(quote!(
                    (#enum_ident::#variant_ident, #enum_ident::#variant_ident)
                )),
            _ => {}
        }
    }
}
