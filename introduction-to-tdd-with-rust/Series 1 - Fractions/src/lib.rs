#[derive(Debug)]
pub struct Fraction {
    pub denominator: i32,
    pub numerator: i32,
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
    pub fn new<T>(arg: T) -> Fraction
    where
        T: Into<Fraction>,
    {
        arg.into()
    }

    pub fn plus(&self, that: &Fraction) -> Fraction {
        Fraction::new((
            self.numerator * that.denominator + that.numerator * self.denominator,
            self.denominator * that.denominator,
        ))
    }

    pub fn multiply(&self, that: &Fraction) -> Fraction {
        Fraction::new((
            self.numerator * that.numerator,
            self.denominator * that.denominator,
        ))
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
    fn zero() {
        assert_eq!(1, gcd(1, 0));
        assert_eq!(5, gcd(0, 5));
        assert_eq!(0, gcd(0, 0));
    }
}
