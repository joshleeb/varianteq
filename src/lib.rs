//! This crate provides a macro to implement equality of enum variants.
//!
//! Two enum variants are equal if they are the same variant from the same enum, regardless of the
//! values of the fields each variant contains.
//!
//! ```no_run
//! # #[macro_use]
//! # extern crate varianteq;
//! #
//! #[derive(VariantEq)]
//! enum Enum {
//!     Variant,
//! }
//! #
//! # fn main() {}
//! ```
//!
//! # Examples
//!
//! ```
//! #[macro_use]
//! extern crate varianteq;
//!
//! #[derive(Debug, VariantEq)]
//! enum E {
//!     A(i32),
//!     B(i32),
//!     C(u32, bool),
//! }
//!
//! fn main() {
//!     assert_eq!(E::A(1), E::A(2));
//!     assert_ne!(E::A(1), E::B(1));
//!     assert_ne!(E::A(1), E::C(1, false));
//! }
//! ```
//!
//! # Errors
//!
//! The `VariantEq` macro only applies to enums and will cauase a compilation error if used on
//! structs.
//!
//! ```compile_fail
//! # #[macro_use]
//! # extern crate varianteq;
//! #
//! #[derive(VariantEq)]
//! struct S;
//! #
//! # fn main() {}
//! ```
//!
//! ```text
//! error: #[derive(VariantEq)] is only defined for enums
//! ```

#![feature(proc_macro_diagnostic)]

extern crate proc_macro;
extern crate proc_macro2;
extern crate syn;

#[macro_use]
extern crate quote;

use proc_macro::{Diagnostic, TokenStream};
use quote::Tokens;
use syn::{parse2, DeriveInput};

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
