extern crate fractions;

use fractions::Fraction;

#[test]
fn different_denominators_without_reducing() {
    assert_eq!(
        Fraction {
            numerator: 5,
            denominator: 6,
        },
        Fraction {
            numerator: 1,
            denominator: 2,
        }.plus(&Fraction {
            numerator: 1,
            denominator: 3,
        })
    );
}

#[test]
fn already_in_lowest_terms() {
    assert_eq!(Fraction::new((3, 4)), Fraction::new((3, 4)));
}

#[test]
fn reduce_to_not_whole_number() {
    assert_eq!(Fraction::new((3, 4)), Fraction::new((6, 8)));
}

#[test]
fn reduce_to_whole_number() {
    assert_eq!(Fraction::new(6), Fraction::new((24, 4)));
}

#[test]
fn reduce_zero() {
    assert_eq!(Fraction::new(0), Fraction::new((0, 172635)));
}

#[test]
fn reduce_result_to_whole_number() {
    assert_eq!(
        Fraction::new(1),
        Fraction::new((1, 3)).plus(&Fraction::new((2, 3)))
    );
}
