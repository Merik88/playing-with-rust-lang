extern crate fractions;

use fractions::Fraction;

#[test]
fn zero_multiply_zero() {
    assert_eq!(
        Fraction::new(0),
        Fraction::new(0).multiply(&Fraction::new(0))
    );
}

#[test]
fn whole_number_multiply_zero() {
    assert_eq!(
        Fraction::new(0),
        Fraction::new(1).multiply(&Fraction::new(0))
    );
}

#[test]
fn one_multiply_three() {
    assert_eq!(
        Fraction::new(3),
        Fraction::new(1).multiply(&Fraction::new(3))
    );
}

#[test]
fn three_multiply_three() {
    assert_eq!(
        Fraction::new(9),
        Fraction::new(3).multiply(&Fraction::new(3))
    );
}

#[test]
fn negative_whole_number_multiply_positive_whole_number() {
    assert_eq!(
        Fraction::new(-3),
        Fraction::new(-1).multiply(&Fraction::new(3))
    );
}

#[test]
fn negative_whole_number_multiply_negative_whole_number() {
    assert_eq!(
        Fraction::new(3),
        Fraction::new(-1).multiply(&Fraction::new(-3))
    );
}

#[test]
fn fractions_with_different_nominators_multiplication() {
    assert_eq!(
        Fraction::new((3, 25)),
        Fraction::new((3, 5)).multiply(&Fraction::new((1, 5)))
    );
}


#[test]
fn fractions_with_different_denominator_multiplication() {
    assert_eq!(
        Fraction::new((9, 5)),
        Fraction::new((3, 1)).multiply(&Fraction::new((3, 5)))
    );
}

#[test]
fn negative_fraction_multiplication() {
    assert_eq!(
        Fraction::new((-3, -25)),
        Fraction::new((-3, 5)).multiply(&Fraction::new((1, -5)))
    );
}

#[test]
fn all_negative_fraction_multiplication() {
    assert_eq!(
        Fraction::new((3, 25)),
        Fraction::new((-3, -5)).multiply(&Fraction::new((-1, -5)))
    );
}
