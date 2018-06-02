# VariantEq

[![Latest Version](https://img.shields.io/badge/crates.io-v0.4.0-orange.svg)](https://crates.io/crates/varianteq)
[![Rust Documentation](https://img.shields.io/badge/api-rustdoc-blue.svg)](https://docs.rs/varianteq)
[![Travis Build](https://travis-ci.org/joshleeb/varianteq.svg?branch=master)](https://travis-ci.org/joshleeb/varianteq)

This crate provides a macro to implement equality of enum variants.

Two enum variants are equal if they are the same variant from the same enum, regardless of the
values of the fields each variant contains.

```rust
#[derive(VariantEq)]
enum Enum {
    Variant,
}
```

## Examples

```rust
#[macro_use]
extern crate varianteq;

#[derive(Debug, VariantEq)]
enum E {
    A(i32),
    B(i32),
    C(u32, bool),
}

fn main() {
    assert_eq!(E::A(1), E::A(2));
    assert_ne!(E::A(1), E::B(1));
    assert_ne!(E::A(1), E::C(1, false));
}
```

## Errors

The `VariantEq` macro only applies to enums and will cauase a compilation error if used on
structs.

```rust
#[derive(VariantEq)]
struct S;
```

```text
error: #[derive(VariantEq)] is only defined for enums
```
