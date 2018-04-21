#[macro_use]
extern crate varianteq;

#[derive(Debug, VariantEq)]
enum Color {
    Red,
    Blue,

    Green(bool),
    Orange(bool),

    Yellow(u32, bool),
    Cyan(u32, bool),
}

#[test]
fn eq_variant_no_fields() {
    assert_eq!(Color::Red, Color::Red);
    assert_eq!(Color::Blue, Color::Blue);
}

#[test]
fn ne_variant_no_fields() {
    assert_ne!(Color::Red, Color::Blue);
}

#[test]
fn eq_variant_unnamed_fields() {
    assert_eq!(Color::Green(true), Color::Green(true));
    assert_eq!(Color::Green(true), Color::Green(false));

    assert_eq!(Color::Orange(true), Color::Orange(true));
    assert_eq!(Color::Orange(true), Color::Orange(false));

    assert_eq!(Color::Yellow(0, true), Color::Yellow(0, true));
    assert_eq!(Color::Yellow(0, true), Color::Yellow(1, false));

    assert_eq!(Color::Cyan(0, true), Color::Cyan(0, true));
    assert_eq!(Color::Cyan(0, true), Color::Cyan(1, false));
}

#[test]
fn ne_variant_unnamed_fields() {
    assert_ne!(Color::Green(true), Color::Orange(true));
    assert_ne!(Color::Green(true), Color::Orange(false));

    assert_ne!(Color::Yellow(0, true), Color::Cyan(0, true));
    assert_ne!(Color::Yellow(0, true), Color::Cyan(1, false));
}

#[test]
fn ne_variant_mixed_fields() {
    assert_ne!(Color::Red, Color::Green(true));
    assert_ne!(Color::Blue, Color::Cyan(0, true));
    assert_ne!(Color::Green(true), Color::Yellow(0, true));
}
