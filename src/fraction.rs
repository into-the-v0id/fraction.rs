use std::ops::{Add, Mul, Div, Sub, Rem};
use std::fmt::{Display, Formatter, Debug};
use std::cmp::max;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
pub struct Fraction<T>
    where T: Mul<Output = T> + Div<Output = T> + Add<Output = T> + Sub<Output = T> + Rem<Output = T> + PartialEq + PartialOrd + Display + Debug + Clone + Copy + From<i32>,
          f64: From<T>
{
    num: T,
    den: T,
}

impl <T> Fraction<T>
    where T: Mul<Output = T> + Div<Output = T> + Add<Output = T> + Sub<Output = T> + Rem<Output = T> + PartialEq + PartialOrd + Display + Debug + Clone + Copy + From<i32>,
          f64: From<T>
{
    pub fn new(num: T, den: T) -> Fraction<T> {
        Fraction::simplify(&Fraction {
            num,
            den
        })
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

    fn simplify(fraction: &Fraction<T>) -> Fraction<T> {
        if fraction.den == T::from(0) {
            return Fraction {
                num: T::from(1),
                den: T::from(0),
            }
        }

        if fraction.num == T::from(0) {
            return Fraction {
                num: T::from(0),
                den: T::from(1),
            }
        }

        let mut fraction = fraction.clone();

        let is_finite = (fraction.num % T::from(1)) == T::from(0);
        if ! is_finite {
            fraction = Fraction::make_parts_finite(&fraction);
        }

        let divisor = Fraction::calc_greatest_common_divisor(&fraction);

        let mut fraction = Fraction {
            num: fraction.num / divisor,
            den: fraction.den / divisor,
        };

        if fraction.den < T::from(0) {
            fraction = Fraction {
                num: fraction.num * T::from(-1),
                den: fraction.den * T::from(-1),
            };
        }

        fraction
    }

    fn calc_greatest_common_divisor(fraction: &Fraction<T>) -> T {
        let mut num = fraction.num;
        let mut den = fraction.den;

        while den != T::from(0) {
            std::mem::swap(&mut num, &mut den);
            den = den % num;
        }

        num
    }

    fn make_parts_finite(fraction: &Fraction<T>) -> Fraction<T> {
        let num_decimal_points = fraction.num.to_string()
            .split('.')
            .skip(1)
            .next()
            .unwrap_or("")
            .len() as i32;

        let den_decimal_points = fraction.den.to_string()
            .split('.')
            .skip(1)
            .next()
            .unwrap_or("")
            .len() as i32;

        let max_decimal_points = max(num_decimal_points, den_decimal_points);
        let multiplier = T::from(10 * max_decimal_points);

        Fraction {
            num: fraction.num * multiplier,
            den: fraction.den * multiplier,
        }
    }

    /// Checks if number has 2 or less places after the decimal point
    pub fn is_simple(&self) -> bool {
        let string = self.to_f64().to_string();
        let mut split = string
            .split('.')
            .skip(1);

        if let Some(decimal) = split.next() {
            return decimal.len() <= 2;
        }

        return true;
    }

    pub fn to_f64(&self) -> f64 {
        self.into()
    }
}

/*
 * Operations
 */
impl <D, T> Add<D> for Fraction<T>
    where D: Into<Fraction<T>>,
          T: Mul<Output = T> + Div<Output = T> + Add<Output = T> + Sub<Output = T> + Rem<Output = T> + PartialEq + PartialOrd + Display + Debug + Clone + Copy + From<i32>,
          f64: From<T>
{
    type Output = Fraction<T>;

    fn add(self, other: D) -> Self::Output {
        let (one, two) = Fraction::sync_base(&self, &other.into());

        Fraction::new(
            one.num + two.num,
            one.den
        )
    }
}

impl <D, T> Sub<D> for Fraction<T>
    where D: Into<Fraction<T>>,
          T: Mul<Output = T> + Div<Output = T> + Add<Output = T> + Sub<Output = T> + Rem<Output = T> + PartialEq + PartialOrd + Display + Debug + Clone + Copy + From<i32>,
          f64: From<T>
{
    type Output = Fraction<T>;

    fn sub(self, other: D) -> Self::Output {
        let (one, two) = Fraction::sync_base(&self, &other.into());

        Fraction::new(
            one.num - two.num,
            one.den
        )
    }
}

impl <D, T> Mul<D> for Fraction<T>
    where D: Into<Fraction<T>>,
          T: Mul<Output = T> + Div<Output = T> + Add<Output = T> + Sub<Output = T> + Rem<Output = T> + PartialEq + PartialOrd + Display + Debug + Clone + Copy + From<i32>,
          f64: From<T>
{
    type Output = Fraction<T>;

    fn mul(self, other: D) -> Self::Output {
        let other = other.into();

        Fraction::new(
            self.num * other.num,
            self.den * other.den
        )
    }
}

impl <D, T> Div<D> for Fraction<T>
    where D: Into<Fraction<T>>,
          T: Mul<Output = T> + Div<Output = T> + Add<Output = T> + Sub<Output = T> + Rem<Output = T> + PartialEq + PartialOrd + Display + Debug + Clone + Copy + From<i32>,
          f64: From<T>
{
    type Output = Fraction<T>;

    fn div(self, other: D) -> Self::Output {
        let other = other.into();

        Fraction::new(
            self.num * other.den,
            self.den * other.num
        )
    }
}

/*
 * Formatting
 */
impl <T> Display for Fraction<T>
    where T: Mul<Output = T> + Div<Output = T> + Add<Output = T> + Sub<Output = T> + Rem<Output = T> + PartialEq + PartialOrd + Display + Debug + Clone + Copy + From<i32>,
          f64: From<T>
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.is_simple() {
            write!(f, "{}", self.to_f64())
        } else {
            write!(f, "{}/{}", self.num, self.den)
        }
    }
}

/*
 * To Number
 */
impl <T> From<&Fraction<T>> for f64
    where T: Mul<Output = T> + Div<Output = T> + Add<Output = T> + Sub<Output = T> + Rem<Output = T> + PartialEq + PartialOrd + Display + Debug + Clone + Copy + From<i32>,
          f64: From<T>
{
    fn from(fr: &Fraction<T>) -> Self {
        f64::from(fr.num) / f64::from(fr.den)
    }
}

impl <T> From<Fraction<T>> for f64
    where T: Mul<Output = T> + Div<Output = T> + Add<Output = T> + Sub<Output = T> + Rem<Output = T> + PartialEq + PartialOrd + Display + Debug + Clone + Copy + From<i32>,
          f64: From<T>
{
    fn from(fr: Fraction<T>) -> Self {
        (&fr).into()
    }
}

impl <T> From<&mut Fraction<T>> for f64
    where T: Mul<Output = T> + Div<Output = T> + Add<Output = T> + Sub<Output = T> + Rem<Output = T> + PartialEq + PartialOrd + Display + Debug + Clone + Copy + From<i32>,
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
    where T: Mul<Output = T> + Div<Output = T> + Add<Output = T> + Sub<Output = T> + Rem<Output = T> + PartialEq + PartialOrd + Display + Debug + Clone + Copy + From<i32>,
          f64: From<T>,
{
    fn from(data: T) -> Self {
        Fraction::new(data, T::from(1))
    }
}

/*
 * Macro
 */
macro_rules! fr {
    ($num:expr) => {
        Fraction::from($num)
    };
    ($num:expr,$den:expr) => {
        Fraction::new($num, $den)
    };
}
