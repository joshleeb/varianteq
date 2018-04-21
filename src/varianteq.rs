use proc_macro::{Diagnostic, Level};
use quote::Tokens;
use syn::{Data, DeriveInput};

pub fn derive(item: DeriveInput) -> Result<Tokens, Diagnostic> {
    let name = item.ident;
    let _data = match item.data {
        Data::Enum(d) => d,
        _ => {
            return Err(Diagnostic::new(
                Level::Error,
                "#[derive(VariantEq)] is only defined for enums",
            ))
        }
    };

    Ok(quote! {
        impl PartialEq for #name {
            fn eq(&self, other: &#name) -> bool {
                true    // TODO(joshleeb): Implement.
            }
        }
        impl Eq for #name {}
    })
}
