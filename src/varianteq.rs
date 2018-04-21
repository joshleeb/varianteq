use proc_macro::Diagnostic;
use quote::Tokens;
use syn::DeriveInput;

pub fn derive(item: DeriveInput) -> Result<Tokens, Diagnostic> {
    let name = item.ident;
    Ok(quote! {
        impl PartialEq for #name {
            fn eq(&self, other: &#name) -> bool {
                true    // TODO(joshleeb): Implement.
            }
        }
        impl Eq for #name {}
    })
}
