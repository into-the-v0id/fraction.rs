use std::ops::Add;
use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
pub struct Fraction {
    num: i64,
    den: i64,
}

impl Fraction {
    pub fn new(num: i64, den: i64) -> Self {
        Fraction {
            num,
            den
        }
    }

    fn sync_base(one: &Fraction, two: &Fraction) -> (Fraction, Fraction) {
        (
            Fraction {
                num: one.num * two.den,
                den: one.den * two.den,
            },
            Fraction {
                num: two.num * one.den,
                den: two.den * one.den,
            }
        )
    }

    /// Checks if number has 3 or less places after the decimal point
    pub fn is_simple(&self) -> bool {
        let number: f64 = self.into();
        let string = number.to_string();
        let mut split = string.split('.');

        split.next();
        let decimal = split.next();

        if let Some(decimal) = decimal {
            return decimal.len() <= 3;
        }

        return false;
    }

    pub fn to_number(&self) -> f64 {
        self.into()
    }
}

impl <T> Add<T> for Fraction
    where T: Into<Fraction>
{
    type Output = Fraction;

    fn add(self, other: T) -> Self::Output {
        let other = other.into();

        let (one, two) = Fraction::sync_base(&self, &other);

        Fraction::new(
            one.num + two.num,
            one.den
        )
    }
}

impl Display for Fraction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.is_simple() {
            write!(f, "{}", self.to_number())
        } else {
            write!(f, "{}/{}", self.num, self.den)
        }
    }
}

/*
 * To Number
 */
impl From<Fraction> for f64 {
    fn from(fr: Fraction) -> Self {
        fr.num as f64 / fr.den as f64
    }
}

impl From<&Fraction> for f64 {
    fn from(fr: &Fraction) -> Self {
        fr.num as f64 / fr.den as f64
    }
}

impl From<&mut Fraction> for f64 {
    fn from(fr: &mut Fraction) -> Self {
        fr.num as f64 / fr.den as f64
    }
}

/*
 * To Fraction
 */
impl From<i64> for Fraction {
    fn from(num: i64) -> Self {
        Fraction::new(num, 1)
    }
}
