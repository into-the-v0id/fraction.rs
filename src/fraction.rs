use std::cmp::max;
use std::convert::TryFrom;
use std::ops::{Add, Mul, Div, Sub, Rem};
use std::fmt::{Display, Formatter, Debug};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
pub struct Fraction<T>
    where T: Mul<Output = T> + Div<Output = T> + Add<Output = T> + Sub<Output = T> + Rem<Output = T> + PartialEq + PartialOrd + Display + Debug + Clone + Copy + From<i8> + Into<f64>
{
    num: T,
    den: T,
}

impl <T> Fraction<T>
    where T: Mul<Output = T> + Div<Output = T> + Add<Output = T> + Sub<Output = T> + Rem<Output = T> + PartialEq + PartialOrd + Display + Debug + Clone + Copy + From<i8> + Into<f64>
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

        let divisor = Fraction::calc_greatest_common_divisor(fraction);

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

    pub fn to_f64(&self) -> f64 {
        self.into()
    }
}

/*
 * Operations
 */
impl <D, T> Add<D> for Fraction<T>
    where D: Into<Fraction<T>>,
          T: Mul<Output = T> + Div<Output = T> + Add<Output = T> + Sub<Output = T> + Rem<Output = T> + PartialEq + PartialOrd + Display + Debug + Clone + Copy + From<i8> + Into<f64>
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
          T: Mul<Output = T> + Div<Output = T> + Add<Output = T> + Sub<Output = T> + Rem<Output = T> + PartialEq + PartialOrd + Display + Debug + Clone + Copy + From<i8> + Into<f64>
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
          T: Mul<Output = T> + Div<Output = T> + Add<Output = T> + Sub<Output = T> + Rem<Output = T> + PartialEq + PartialOrd + Display + Debug + Clone + Copy + From<i8> + Into<f64>
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
          T: Mul<Output = T> + Div<Output = T> + Add<Output = T> + Sub<Output = T> + Rem<Output = T> + PartialEq + PartialOrd + Display + Debug + Clone + Copy + From<i8> + Into<f64>
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

impl <D, T> Rem<D> for Fraction<T>
    where D: Into<Fraction<T>>,
          T: Mul<Output = T> + Div<Output = T> + Add<Output = T> + Sub<Output = T> + Rem<Output = T> + PartialEq + PartialOrd + Display + Debug + Clone + Copy + From<i8> + Into<f64>
{
    type Output = Fraction<T>;

    fn rem(self, other: D) -> Self::Output {
        let (one, two) = Fraction::sync_base(&self, &other.into());

        Fraction::new(
            one.num % two.num,
            one.den
        )
    }
}

/*
 * Formatting
 */
impl <T> Display for Fraction<T>
    where T: Mul<Output = T> + Div<Output = T> + Add<Output = T> + Sub<Output = T> + Rem<Output = T> + PartialEq + PartialOrd + Display + Debug + Clone + Copy + From<i8> + Into<f64>
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.num, self.den)
    }
}

/*
 * To Number
 */
impl <T> From<&Fraction<T>> for f64
    where T: Mul<Output = T> + Div<Output = T> + Add<Output = T> + Sub<Output = T> + Rem<Output = T> + PartialEq + PartialOrd + Display + Debug + Clone + Copy + From<i8> + Into<f64>
{
    fn from(fr: &Fraction<T>) -> Self {
        fr.num.into() / fr.den.into()
    }
}

impl <T> From<Fraction<T>> for f64
    where T: Mul<Output = T> + Div<Output = T> + Add<Output = T> + Sub<Output = T> + Rem<Output = T> + PartialEq + PartialOrd + Display + Debug + Clone + Copy + From<i8> + Into<f64>
{
    fn from(fr: Fraction<T>) -> Self {
        (&fr).into()
    }
}

impl <T> From<&mut Fraction<T>> for f64
    where T: Mul<Output = T> + Div<Output = T> + Add<Output = T> + Sub<Output = T> + Rem<Output = T> + PartialEq + PartialOrd + Display + Debug + Clone + Copy + From<i8> + Into<f64>
{
    fn from(fr: &mut Fraction<T>) -> Self {
        (&*fr).into()
    }
}

/*
 * To Fraction
 */
impl <T> From<T> for Fraction<T>
    where T: Mul<Output = T> + Div<Output = T> + Add<Output = T> + Sub<Output = T> + Rem<Output = T> + PartialEq + PartialOrd + Display + Debug + Clone + Copy + From<i8> + Into<f64>,
          T: Into<i128> // only implement for integers
{
    fn from(data: T) -> Self {
        Fraction::new(data, T::from(1))
    }
}

impl TryFrom<f64> for Fraction<i32> {
    type Error = ();

    fn try_from(data: f64) -> Result<Self, Self::Error> {
        let mut data_number_string = data.to_string()
            .replace('.', "");
        let mut max_number_string = i32::MAX.to_string();
        let max_len = max(data_number_string.len(), max_number_string.len());
        data_number_string = format!("{:0>len$}", data_number_string, len=max_len);
        max_number_string = format!("{:0>len$}", max_number_string, len=max_len);

        if data_number_string > max_number_string {
            return Err(());
        }

        let decimal_points = data.to_string()
            .split('.')
            .skip(1)
            .next()
            .unwrap_or("")
            .len();

        let multiplier = (10 as i32).pow(decimal_points as u32);

        Ok(
            Fraction::new(
                (data * (multiplier as f64)) as i32,
                multiplier,
            )
        )
    }
}

impl TryFrom<f32> for Fraction<i32> {
    type Error = ();

    fn try_from(data: f32) -> Result<Self, Self::Error> {
        Fraction::try_from(data as f64)
    }
}

/*
 * Macro
 */
macro_rules! fr {
    ($num:literal/$den:literal) => {
        Fraction::from($num) / Fraction::from($den)
    };
    ($num:expr,$den:expr) => {
        Fraction::from($num) / Fraction::from($den)
    };
    ($num:expr) => {
        Fraction::from($num)
    };
}
