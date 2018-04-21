#[macro_use]
extern crate varianteq;

#[derive(Debug, VariantEq)]
enum Color {
    Red,
}

#[test]
fn eq_variant_no_fields() {
    assert_eq!(Color::Red, Color::Red)
}
