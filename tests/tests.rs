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

    Purple { a: bool },
    Indigo { a: bool },

    Black { a: u32, b: bool },
    White { a: u32, b: bool },
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

    assert_eq!(Color::Yellow(0, true), Color::Yellow(0, true));
    assert_eq!(Color::Yellow(0, true), Color::Yellow(1, false));
}

#[test]
fn ne_variant_unnamed_fields() {
    assert_ne!(Color::Green(true), Color::Orange(true));
    assert_ne!(Color::Green(true), Color::Orange(false));

    assert_ne!(Color::Yellow(0, true), Color::Cyan(0, true));
    assert_ne!(Color::Yellow(0, true), Color::Cyan(1, false));
}

#[test]
fn eq_variant_named_fields() {
    assert_eq!(Color::Purple { a: true }, Color::Purple { a: true });
    assert_eq!(Color::Purple { a: true }, Color::Purple { a: false });

    assert_eq!(
        Color::Black { a: 0, b: true },
        Color::Black { a: 0, b: true }
    );
    assert_eq!(
        Color::Black { a: 0, b: true },
        Color::Black { a: 1, b: false }
    );
}

#[test]
fn ne_variant_named_fields() {
    assert_ne!(Color::Purple { a: true }, Color::Indigo { a: true });
    assert_ne!(Color::Purple { a: true }, Color::Indigo { a: false });

    assert_ne!(
        Color::Black { a: 0, b: true },
        Color::White { a: 0, b: true }
    );
    assert_ne!(
        Color::Black { a: 0, b: true },
        Color::White { a: 1, b: false }
    );
}

#[test]
fn ne_variant_mixed_fields() {
    assert_ne!(Color::Red, Color::Green(true));
    assert_ne!(Color::Blue, Color::Cyan(0, true));
    assert_ne!(Color::Green(true), Color::Yellow(0, true));
    assert_ne!(Color::Purple { a: true }, Color::Black { a: 0, b: true });
    assert_ne!(Color::Blue, Color::White { a: 0, b: true });
    assert_ne!(Color::Black { a: 0, b: false }, Color::Yellow(0, true));
}
