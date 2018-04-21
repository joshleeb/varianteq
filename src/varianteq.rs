use proc_macro::{Diagnostic, Level};
use quote::Tokens;
use syn::{Data, DeriveInput};
use token::EnumVariant;

pub fn derive(item: DeriveInput) -> Result<Tokens, Diagnostic> {
    let name = item.ident;
    let data = match item.data {
        Data::Enum(d) => d,
        _ => {
            return Err(Diagnostic::new(
                Level::Error,
                "#[derive(VariantEq)] is only defined for enums",
            ))
        }
    };

    let mut enum_variants = vec![];
    for variant in data.variants {
        enum_variants.push(EnumVariant::new(&item.ident, variant));
    }

    Ok(quote! {
        impl PartialEq for #name {
            fn eq(&self, other: &#name) -> bool {
                match (self, other) {
                    #(#enum_variants => true,)*
                    _ => false,
                }
            }
        }
        impl Eq for #name {}
    })
}
