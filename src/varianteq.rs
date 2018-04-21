use proc_macro2::TokenStream;
use quote::Tokens;
use syn::{parse2, DeriveInput};

pub fn derive(tokens: TokenStream) -> Tokens {
    let ast: DeriveInput = parse2(tokens).unwrap();
    let name = ast.ident;

    quote! {
        impl PartialEq for #name {
            fn eq(&self, other: &#name) -> bool {
                true    // TODO(joshleeb): Implement.
            }
        }
        impl Eq for #name {}
    }
}
