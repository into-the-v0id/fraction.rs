use std::ops::{Add, Mul, Div, Sub};
use std::fmt::{Display, Formatter, Debug};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
pub struct Fraction<T = i32>
    where T: Mul<Output = T> + Div<Output = T> + Add<Output = T> + Sub<Output = T> + Display + Debug + Clone + Copy,
          f64: From<T>
{
    num: T,
    den: T,
}

impl <T> Fraction<T>
    where T: Mul<Output = T> + Div<Output = T> + Add<Output = T> + Sub<Output = T> + Display + Debug + Clone + Copy,
          f64: From<T>
{
    pub fn new(num: T, den: T) -> Fraction<T> {
        Fraction {
            num,
            den
        }
    }

    fn sync_base(one: &Fraction<T>, two: &Fraction<T>) -> (Fraction<T>, Fraction<T>) {
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

impl <D, T> Add<D> for Fraction<T>
    where D: Into<Fraction<T>>,
          T: Mul<Output = T> + Div<Output = T> + Add<Output = T> + Sub<Output = T> + Display + Debug + Clone + Copy,
          f64: From<T>
{
    type Output = Fraction<T>;

    fn add(self, other: D) -> Self::Output {
        let other = other.into();

        let (one, two) = Fraction::sync_base(&self, &other);

        Fraction::new(
            one.num + two.num,
            one.den
        )
    }
}

impl <T> Display for Fraction<T>
    where T: Mul<Output = T> + Div<Output = T> + Add<Output = T> + Sub<Output = T> + Display + Debug + Clone + Copy,
          f64: From<T>
{
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
impl <T> From<&Fraction<T>> for f64
    where T: Mul<Output = T> + Div<Output = T> + Add<Output = T> + Sub<Output = T> + Display + Debug + Clone + Copy,
          f64: From<T>
{
    fn from(fr: &Fraction<T>) -> Self {
        f64::from(fr.num) / f64::from(fr.den)
    }
}

impl <T> From<Fraction<T>> for f64
    where T: Mul<Output = T> + Div<Output = T> + Add<Output = T> + Sub<Output = T> + Display + Debug + Clone + Copy,
          f64: From<T>
{
    fn from(fr: Fraction<T>) -> Self {
        (&fr).into()
    }
}

impl <T> From<&mut Fraction<T>> for f64
    where T: Mul<Output = T> + Div<Output = T> + Add<Output = T> + Sub<Output = T> + Display + Debug + Clone + Copy,
          f64: From<T>
{
    fn from(fr: &mut Fraction<T>) -> Self {
        (&*fr).into()
    }
}

/*
 * To Fraction
 */
impl <T> From<T> for Fraction<T>
    where T: Mul<Output = T> + Div<Output = T> + Add<Output = T> + Sub<Output = T> + Display + Debug + Clone + Copy + From<u8>,
          f64: From<T>,
{
    fn from(data: T) -> Self {
        Fraction::new(data, T::from(1))
    }
}
