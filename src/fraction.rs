use std::convert::TryFrom;
use std::ops::{Add, Mul, Div, Sub, Rem};
use std::fmt::{Display, Formatter, Debug};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
pub struct Fraction<T>
    where T: Mul<Output = T> + Div<Output = T> + Add<Output = T> + Sub<Output = T> + Rem<Output = T> + PartialEq + PartialOrd + Display + Debug + Clone + Copy + From<i8>
{
    num: T,
    den: T,
}

impl <T> Fraction<T>
    where T: Mul<Output = T> + Div<Output = T> + Add<Output = T> + Sub<Output = T> + Rem<Output = T> + PartialEq + PartialOrd + Display + Debug + Clone + Copy + From<i8>
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
}

/*
 * Operations
 */
impl <D, T> Add<D> for Fraction<T>
    where D: Into<Fraction<T>>,
          T: Mul<Output = T> + Div<Output = T> + Add<Output = T> + Sub<Output = T> + Rem<Output = T> + PartialEq + PartialOrd + Display + Debug + Clone + Copy + From<i8>
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
          T: Mul<Output = T> + Div<Output = T> + Add<Output = T> + Sub<Output = T> + Rem<Output = T> + PartialEq + PartialOrd + Display + Debug + Clone + Copy + From<i8>
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
          T: Mul<Output = T> + Div<Output = T> + Add<Output = T> + Sub<Output = T> + Rem<Output = T> + PartialEq + PartialOrd + Display + Debug + Clone + Copy + From<i8>
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
          T: Mul<Output = T> + Div<Output = T> + Add<Output = T> + Sub<Output = T> + Rem<Output = T> + PartialEq + PartialOrd + Display + Debug + Clone + Copy + From<i8>
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
          T: Mul<Output = T> + Div<Output = T> + Add<Output = T> + Sub<Output = T> + Rem<Output = T> + PartialEq + PartialOrd + Display + Debug + Clone + Copy + From<i8>
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
    where T: Mul<Output = T> + Div<Output = T> + Add<Output = T> + Sub<Output = T> + Rem<Output = T> + PartialEq + PartialOrd + Display + Debug + Clone + Copy + From<i8>
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.num, self.den)
    }
}

/*
 * To Float
 */
impl <T> From<&Fraction<T>> for f64
    where T: Mul<Output = T> + Div<Output = T> + Add<Output = T> + Sub<Output = T> + Rem<Output = T> + PartialEq + PartialOrd + Display + Debug + Clone + Copy + From<i8>,
          T: Into<f64>
{
    fn from(fr: &Fraction<T>) -> Self {
        fr.num.into() / fr.den.into()
    }
}

impl <T> From<Fraction<T>> for f64
    where T: Mul<Output = T> + Div<Output = T> + Add<Output = T> + Sub<Output = T> + Rem<Output = T> + PartialEq + PartialOrd + Display + Debug + Clone + Copy + From<i8>,
          T: Into<f64>
{
    fn from(fr: Fraction<T>) -> Self {
        (&fr).into()
    }
}

impl <T> From<&mut Fraction<T>> for f64
    where T: Mul<Output = T> + Div<Output = T> + Add<Output = T> + Sub<Output = T> + Rem<Output = T> + PartialEq + PartialOrd + Display + Debug + Clone + Copy + From<i8>,
          T: Into<f64>
{
    fn from(fr: &mut Fraction<T>) -> Self {
        (&*fr).into()
    }
}

impl <T> From<&Fraction<T>> for f32
    where T: Mul<Output = T> + Div<Output = T> + Add<Output = T> + Sub<Output = T> + Rem<Output = T> + PartialEq + PartialOrd + Display + Debug + Clone + Copy + From<i8>,
          T: Into<f32>
{
    fn from(fr: &Fraction<T>) -> Self {
        fr.num.into() / fr.den.into()
    }
}

impl <T> From<Fraction<T>> for f32
    where T: Mul<Output = T> + Div<Output = T> + Add<Output = T> + Sub<Output = T> + Rem<Output = T> + PartialEq + PartialOrd + Display + Debug + Clone + Copy + From<i8>,
          T: Into<f32>
{
    fn from(fr: Fraction<T>) -> Self {
        (&fr).into()
    }
}

impl <T> From<&mut Fraction<T>> for f32
    where T: Mul<Output = T> + Div<Output = T> + Add<Output = T> + Sub<Output = T> + Rem<Output = T> + PartialEq + PartialOrd + Display + Debug + Clone + Copy + From<i8>,
          T: Into<f32>
{
    fn from(fr: &mut Fraction<T>) -> Self {
        (&*fr).into()
    }
}

/*
 * To Fraction
 */
impl <T> From<T> for Fraction<T>
    where T: Mul<Output = T> + Div<Output = T> + Add<Output = T> + Sub<Output = T> + Rem<Output = T> + PartialEq + PartialOrd + Display + Debug + Clone + Copy + From<i8>,
          T: Into<i128> // only implement for integers
{
    fn from(data: T) -> Self {
        Fraction::new(data, T::from(1))
    }
}

impl TryFrom<f64> for Fraction<i128> {
    type Error = ();

    fn try_from(data: f64) -> Result<Self, Self::Error> {
        let mut num = data;
        let mut den: i128 = 1;

        while num % 1.0 != 0.0 {
            // lossy multiplication guard
            if num > f64::MAX / 10.0 || den > i128::MAX / 10 {
                return Err(());
            }

            num *= 10.0;
            den *= 10;
        }

        // lossy conversion guard
        if num > i128::MAX as f64 {
            return Err(());
        }

        Ok(Fraction::new(num as i128, den))
    }
}

impl TryFrom<f32> for Fraction<i128> {
    type Error = <Self as TryFrom<f64>>::Error;

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
