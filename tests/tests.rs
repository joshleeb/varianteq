#[macro_use]
extern crate varianteq;

#[derive(Debug, VariantEq)]
enum Color {
    Red,
    Blue,
    // Green(u32),
    // Yellow({s: String, b: bool}),
}

#[test]
fn eq_variant_no_fields() {
    assert_eq!(Color::Red, Color::Red);
    assert_eq!(Color::Blue, Color::Blue);
}

#[test]
fn ne_variant_no_fields() {
    assert_ne!(Color::Red, Color::Blue);
    assert_ne!(Color::Blue, Color::Red);
}
