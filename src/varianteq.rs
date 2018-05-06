use proc_macro::{Diagnostic, Level};
use quote::Tokens;
use syn::{Data, DeriveInput};

pub fn derive(item: DeriveInput) -> Result<Tokens, Diagnostic> {
    check_enum_data(item.data)?;

    let ident = item.ident;
    Ok(quote! {
        impl PartialEq for #ident {
            fn eq(&self, other: &#ident) -> bool {
                ::std::mem::discriminant(self) == ::std::mem::discriminant(other)
            }
        }
        impl Eq for #ident {}
    })
}

fn check_enum_data(data: Data) -> Result<(), Diagnostic> {
    match data {
        Data::Enum(_) => Ok(()),
        _ => Err(Diagnostic::new(
            Level::Error,
            "#[derive(VariantEq)] is only defined for enums",
        )),
    }
}
