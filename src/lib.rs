#![feature(proc_macro)]

#[macro_use]
extern crate quote;

extern crate proc_macro;
extern crate proc_macro2;
extern crate syn;

use proc_macro::{Diagnostic, TokenStream};
use quote::Tokens;
use syn::{parse2, DeriveInput};

mod token;
mod varianteq;

type DeriveFn = fn(DeriveInput) -> Result<Tokens, Diagnostic>;

#[proc_macro_derive(VariantEq)]
pub fn varianteq_derive(tokens: TokenStream) -> TokenStream {
    expand_derive(tokens, varianteq::derive)
}

fn expand_derive(tokens: TokenStream, derive: DeriveFn) -> TokenStream {
    let item = parse2(tokens.into()).unwrap();
    match derive(item) {
        Ok(tokens) => tokens.into(),
        Err(err) => handle_derive_err(err),
    }
}

fn handle_derive_err(err: Diagnostic) -> TokenStream {
    err.emit();
    "".parse().unwrap()
}
