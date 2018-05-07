use proc_macro::{Diagnostic, Level};
use quote::Tokens;
use syn::{Data, DeriveInput};

pub fn derive(item: DeriveInput) -> Result<Tokens, Diagnostic> {
    check_enum_data(item.data)?;

    let ident = item.ident;
    let (impl_generics, ty_generics, where_clause) = item.generics.split_for_impl();

    Ok(quote! {
        impl #impl_generics PartialEq for #ident #ty_generics #where_clause {
            fn eq(&self, other: &#ident) -> bool {
                ::std::mem::discriminant(self) == ::std::mem::discriminant(other)
            }
        }
        impl #impl_generics Eq for #ident #ty_generics #where_clause {}
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
