extern crate fractions;

use fractions::Fraction;

#[test]
fn zero_plus_zero() {
    assert_eq!(
        Fraction {
            numerator: 0,
            ..Default::default()
        },
        Fraction {
            numerator: 0,
            ..Default::default()
        }.plus(&Fraction {
            numerator: 0,
            ..Default::default()
        })
    );
}

#[test]
fn non_zero_plus_zero() {
    assert_eq!(
        Fraction {
            numerator: 3,
            ..Default::default()
        },
        Fraction {
            numerator: 3,
            ..Default::default()
        }.plus(&Fraction {
            numerator: 0,
            ..Default::default()
        })
    );
}

#[test]
fn zero_plus_non_zero() {
    assert_eq!(
        Fraction {
            numerator: 5,
            ..Default::default()
        },
        Fraction {
            numerator: 0,
            ..Default::default()
        }.plus(&Fraction {
            numerator: 5,
            ..Default::default()
        })
    );
}

#[test]
fn non_negative_non_zero_operands() {
    assert_eq!(
        Fraction {
            numerator: 7,
            ..Default::default()
        },
        Fraction {
            numerator: 3,
            ..Default::default()
        }.plus(&Fraction {
            numerator: 4,
            ..Default::default()
        })
    );
}

#[test]
fn negative_inputs_and_negative_output() {
    assert_eq!(
        Fraction {
            numerator: -2,
            ..Default::default()
        },
        Fraction {
            numerator: -3,
            ..Default::default()
        }.plus(&Fraction {
            numerator: 1,
            ..Default::default()
        })
    );
}

#[test]
fn non_trivial_denominator_but_both_same_value() {
    assert_eq!(
        Fraction {
            numerator: 3,
            denominator: 5,
        },
        Fraction {
            denominator: 5,
            numerator: 1,
        }.plus(&Fraction {
            denominator: 5,
            numerator: 2,
        })
    );
}

#[test]
fn one_denominator_is_multiply_of_other() {
    assert_eq!(
        Fraction::new((11, 8)),
        Fraction::new((3, 4)).plus(&Fraction::new((5, 8)))
    );
}

#[test]
fn common_factor_in_denominators() {
    assert_eq!(
        Fraction::new((11, 18)),
        Fraction::new((1, 6)).plus(&Fraction::new((4, 9)))
    );
}

#[test]
fn reduce_result_even_when_denominators_are_the_same() {
    assert_eq!(
        Fraction::new((3, 2)),
        Fraction::new((3, 4)).plus(&Fraction::new((3, 4)))
    );
}

#[test]
fn negative_fraction_and_reducing() {
    assert_eq!(
        Fraction::new((1, 2)),
        Fraction::new((-1, 4)).plus(&Fraction::new((3, 4)))
    );
    assert_eq!(
        Fraction::new((-1, 8)),
        Fraction::new((3, 8)).plus(&Fraction::new((-1, 2)))
    );
}

#[test]
fn negative_signs_everywhere() {
    assert_eq!(
        Fraction::new((1, 2)),
        Fraction::new((1, -4)).plus(&Fraction::new((-3, -4)))
    );
}
