#[derive(Debug, VariantEq)]
enum GenericColor<A: ToOwned> {
    Red(A),
    Blue(A),
}

#[test]
fn eq_generic_variants() {
    assert_eq!(GenericColor::Red(true), GenericColor::Red(true));
    assert_eq!(GenericColor::Red(true), GenericColor::Red(false));

    assert_eq!(GenericColor::Blue(1), GenericColor::Blue(1));
    assert_eq!(GenericColor::Blue(1), GenericColor::Blue(2));
}

#[test]
fn ne_generic_variants() {
    assert_ne!(GenericColor::Red(1), GenericColor::Blue(1));
}

#[derive(Debug, VariantEq)]
enum GenericColorExplicit<A: ToOwned, B = u32> {
    Red(A),
    Blue(B),
}

#[test]
fn eq_generic_variants_explicit() {
    assert_eq!(
        GenericColorExplicit::Red::<bool, u32>(true),
        GenericColorExplicit::Red::<bool, u32>(true)
    );
    assert_eq!(
        GenericColorExplicit::Red::<bool, u32>(true),
        GenericColorExplicit::Red::<bool, u32>(false)
    );

    assert_eq!(
        GenericColorExplicit::Blue::<bool, u32>(1),
        GenericColorExplicit::Blue::<bool, u32>(1)
    );
    assert_eq!(
        GenericColorExplicit::Blue::<bool, u32>(1),
        GenericColorExplicit::Blue::<bool, u32>(2)
    );
}

#[test]
fn ne_generic_variants_explicit() {
    assert_ne!(
        GenericColorExplicit::Red::<u32, u32>(1),
        GenericColorExplicit::Blue::<u32, u32>(1)
    );
}
