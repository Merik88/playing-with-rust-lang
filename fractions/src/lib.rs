#[derive(Debug)]
struct Fraction {
    denominator: i32,
    numerator: i32,
}

impl Default for Fraction {
    fn default() -> Fraction {
        Fraction {
            denominator: 1,
            numerator: 0,
        }
    }
}

impl PartialEq for Fraction {
    fn eq(&self, other: &Fraction) -> bool {
        self.numerator == other.numerator && self.denominator == other.denominator
    }
}

impl Fraction {
    fn new<T>(arg: T) -> Fraction
        where T: Into<Fraction>
    {
        arg.into()
    }

    fn plus(&self, that: &Fraction) -> Fraction {
        Fraction::new((self.numerator * that.denominator + that.numerator * self.denominator,
                       self.denominator * that.denominator))
    }
}

impl From<i32> for Fraction {
    fn from(a: i32) -> Fraction {
        let f = Fraction {
            numerator: a,
            ..Default::default()
        };
        let gcd = gcd(f.numerator, f.denominator);
        Fraction {
            numerator: a / gcd,
            denominator: f.denominator / gcd,
        }
    }
}

impl From<(i32, i32)> for Fraction {
    fn from((a, b): (i32, i32)) -> Fraction {
        let gcd = gcd(a, b);
        Fraction {
            numerator: a / gcd,
            denominator: b / gcd,
        }
    }
}

fn gcd(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        let t = b;
        b = a % t;
        a = t;
    }
    a.abs()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_plus_zero() {
        assert_eq!(Fraction {
                       numerator: 0,
                       ..Default::default()
                   },
                   Fraction {
                           numerator: 0,
                           ..Default::default()
                       }
                       .plus(&Fraction {
                                  numerator: 0,
                                  ..Default::default()
                              }));
    }

    #[test]
    fn non_zero_plus_zero() {
        assert_eq!(Fraction {
                       numerator: 3,
                       ..Default::default()
                   },
                   Fraction {
                           numerator: 3,
                           ..Default::default()
                       }
                       .plus(&Fraction {
                                  numerator: 0,
                                  ..Default::default()
                              }));
    }

    #[test]
    fn zero_plus_non_zero() {
        assert_eq!(Fraction {
                       numerator: 5,
                       ..Default::default()
                   },
                   Fraction {
                           numerator: 0,
                           ..Default::default()
                       }
                       .plus(&Fraction {
                                  numerator: 5,
                                  ..Default::default()
                              }));
    }

    #[test]
    fn non_negative_non_zero_operands() {
        assert_eq!(Fraction {
                       numerator: 7,
                       ..Default::default()
                   },
                   Fraction {
                           numerator: 3,
                           ..Default::default()
                       }
                       .plus(&Fraction {
                                  numerator: 4,
                                  ..Default::default()
                              }));
    }

    #[test]
    fn negative_inputs_and_negative_output() {
        assert_eq!(Fraction {
                       numerator: -2,
                       ..Default::default()
                   },
                   Fraction {
                           numerator: -3,
                           ..Default::default()
                       }
                       .plus(&Fraction {
                                  numerator: 1,
                                  ..Default::default()
                              }));
    }

    #[test]
    fn non_trivial_denominator_but_both_same_value() {
        assert_eq!(Fraction {
                       numerator: 3,
                       denominator: 5,
                   },
                   Fraction {
                           denominator: 5,
                           numerator: 1,
                       }
                       .plus(&Fraction {
                                  denominator: 5,
                                  numerator: 2,
                              }));
    }

    #[test]
    fn same_numerator_and_denominator() {
        let this = Fraction {
            numerator: 3,
            denominator: 5,
        };
        let that = Fraction {
            numerator: 3,
            denominator: 5,
        };
        assert_eq!(this, that);
    }

    #[test]
    fn different_numerator() {
        let this = Fraction {
            numerator: 1,
            denominator: 5,
        };
        let that = Fraction {
            numerator: 2,
            denominator: 5,
        };
        assert!(this != that);
    }

    #[test]
    fn different_denominator() {
        let this = Fraction {
            numerator: 3,
            denominator: 4,
        };
        let that = Fraction {
            numerator: 3,
            denominator: 7,
        };
        assert!(this != that);
    }

    #[test]
    fn whole_number_equals_same_fraction() {
        let this = Fraction {
            numerator: 5,
            denominator: 1,
        };
        let that = Fraction {
            numerator: 5,
            ..Default::default()
        };
        assert!(this == that);
    }

    #[test]
    fn whole_number_not_equal_to_different_whole_number() {
        let this = Fraction {
            numerator: 6,
            ..Default::default()
        };
        let that = Fraction {
            numerator: 5,
            ..Default::default()
        };
        assert!(this != that);
    }

    #[test]
    fn one_argument_constructor() {
        let Fraction {
            numerator: num,
            denominator: den,
        } = Fraction::new(1);
        assert_eq!(1, num);
        assert_eq!(1, den);
    }

    #[test]
    fn two_argument_constructor() {
        let Fraction {
            numerator: num,
            denominator: den,
        } = Fraction::new((3, 4));
        assert_eq!(3, num);
        assert_eq!(4, den);
    }

    #[test]
    fn different_denominators_without_reducing() {
        assert_eq!(Fraction {
                       numerator: 5,
                       denominator: 6,
                   },
                   Fraction {
                           numerator: 1,
                           denominator: 2,
                       }
                       .plus(&Fraction {
                                  numerator: 1,
                                  denominator: 3,
                              }));
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
        assert_eq!(Fraction::new(1),
                   Fraction::new((1, 3)).plus(&Fraction::new((2, 3))));
    }

    #[test]
    fn reflexive() {
        assert_eq!(1, gcd(1, 1));
        assert_eq!(2, gcd(2, 2));
        assert_eq!(1, gcd(-1, -1));
    }

    #[test]
    fn relatively_prime() {
        assert_eq!(1, gcd(2, 3));
        assert_eq!(1, gcd(4, 7));
        assert_eq!(1, gcd(-2, -3));
    }

    #[test]
    fn one_is_multiple_of_other() {
        assert_eq!(3, gcd(3, 9));
        assert_eq!(5, gcd(5, 30));
    }

    #[test]
    fn common_factor() {
        assert_eq!(2, gcd(6, 8));
        assert_eq!(7, gcd(49, 315));
        assert_eq!(4, gcd(-24, -28));
    }

    #[test]
    fn negatives() {
        assert_eq!(4, gcd(-24, 28));
        assert_eq!(4, gcd(24, -28));
    }

    #[test]
    fn one_denominator_is_multiply_of_other() {
        assert_eq!(Fraction::new((11, 8)),
                   Fraction::new((3, 4)).plus(&Fraction::new((5, 8))));
    }

    #[test]
    fn common_factor_in_denominators() {
        assert_eq!(Fraction::new((11, 18)),
                   Fraction::new((1, 6)).plus(&Fraction::new((4, 9))));
    }

    #[test]
    fn reduce_result_even_when_denominators_are_the_same() {
        assert_eq!(Fraction::new((3, 2)),
                   Fraction::new((3, 4)).plus(&Fraction::new((3, 4))));
    }

    #[test]
    fn negative_fraction_and_reducing() {
        assert_eq!(Fraction::new((1, 2)),
                   Fraction::new((-1, 4)).plus(&Fraction::new((3, 4))));
        assert_eq!(Fraction::new((-1, 8)),
                   Fraction::new((3, 8)).plus(&Fraction::new((-1, 2))));
    }

    #[test]
    fn negative_signs_everywhere() {
        assert_eq!(Fraction::new((1, 2)),
                   Fraction::new((1, -4)).plus(&Fraction::new((-3, -4))));
    }

    #[test]
    fn zero() {
        assert_eq!(1, gcd(1, 0));
        assert_eq!(5, gcd(0, 5));
        assert_eq!(0, gcd(0, 0));
    }
}
