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
impl <T> From<Fraction<T>> for f64
    where T: Mul<Output = T> + Div<Output = T> + Add<Output = T> + Sub<Output = T> + Display + Debug + Clone + Copy,
          f64: From<T>
{
    fn from(fr: Fraction<T>) -> Self {
        // f64::from(fr.num) / f64::from(fr.den)
        (&fr).into()
    }
}

impl <T> From<&Fraction<T>> for f64
    where T: Mul<Output = T> + Div<Output = T> + Add<Output = T> + Sub<Output = T> + Display + Debug + Clone + Copy,
          f64: From<T>
{
    fn from(fr: &Fraction<T>) -> Self {
        f64::from(fr.num) / f64::from(fr.den)
    }
}

impl <T> From<&mut Fraction<T>> for f64
    where T: Mul<Output = T> + Div<Output = T> + Add<Output = T> + Sub<Output = T> + Display + Debug + Clone + Copy,
          f64: From<T>
{
    fn from(fr: &mut Fraction<T>) -> Self {
        // f64::from(fr.num) / f64::from(fr.den)
        (&*fr).into()
    }
}

/*
 * To Fraction
 */
impl From<i32> for Fraction<i32> {
    fn from(num: i32) -> Self {
        Fraction::new(num, 1)
    }
}

impl From<i16> for Fraction<i16> {
    fn from(num: i16) -> Self {
        Fraction::new(num, 1)
    }
}

impl From<i8> for Fraction<i8> {
    fn from(num: i8) -> Self {
        Fraction::new(num, 1)
    }
}

impl From<u8> for Fraction<u8> {
    fn from(num: u8) -> Self {
        Fraction::new(num, 1)
    }
}

impl From<u16> for Fraction<u16> {
    fn from(num: u16) -> Self {
        Fraction::new(num, 1)
    }
}

impl From<u32> for Fraction<u32> {
    fn from(num: u32) -> Self {
        Fraction::new(num, 1)
    }
}
