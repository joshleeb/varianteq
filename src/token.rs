use quote::{ToTokens, Tokens};
use syn::{Ident, Variant};

pub struct EnumVariant<'a> {
    enum_ident: &'a Ident,
    variant: &'a Variant,
}

impl<'a> EnumVariant<'a> {
    pub fn new(enum_ident: &'a Ident, variant: &'a Variant) -> Self {
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

        tokens.append_all(quote! {
            (#enum_ident::#variant_ident{..}, #enum_ident::#variant_ident{..})
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::Fields;

    #[test]
    fn to_tokens_unit_field() {
        let enum_ident = Ident::from("Enum");

        let variant = new_variant(Ident::from("VarA"), Fields::Unit);
        let enum_variant = EnumVariant::new(&enum_ident, &variant);

        let mut tokens = Tokens::new();
        enum_variant.to_tokens(&mut tokens);

        assert_eq!(quote!((Enum::VarA { .. }, Enum::VarA { .. })), tokens);
    }

    fn new_variant(ident: Ident, fields: Fields) -> Variant {
        Variant {
            attrs: vec![],
            ident: ident,
            fields: fields,
            discriminant: None,
        }
    }
}
