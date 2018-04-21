#![feature(proc_macro)]

#[macro_use]
extern crate quote;

extern crate proc_macro;
extern crate proc_macro2;
extern crate syn;

use proc_macro::TokenStream;

mod varianteq;

#[proc_macro_derive(VariantEq)]
pub fn varianteq_derive(tokens: TokenStream) -> TokenStream {
    varianteq::derive(tokens.into()).into()
}
