use proc_macro::{Diagnostic, Level};
use quote::Tokens;
use syn::{Data, DataEnum, DeriveInput};
use token::EnumVariant;

pub fn derive(item: DeriveInput) -> Result<Tokens, Diagnostic> {
    let ident = item.ident;
    let data = enum_data(item.data)?;
    let enum_variants: Vec<EnumVariant> = data.variants
        .iter()
        .map(|variant| EnumVariant::new(&ident, variant))
        .collect();

    Ok(quote! {
        impl PartialEq for #ident {
            fn eq(&self, other: &#ident) -> bool {
                match (self, other) {
                    #(#enum_variants => true,)*
                    _ => false,
                }
            }
        }
        impl Eq for #ident {}
    })
}

fn enum_data(data: Data) -> Result<DataEnum, Diagnostic> {
    match data {
        Data::Enum(d) => Ok(d),
        _ => {
            return Err(Diagnostic::new(
                Level::Error,
                "#[derive(VariantEq)] is only defined for enums",
            ))
        }
    }
}
