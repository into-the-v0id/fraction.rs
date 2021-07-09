use crate::fraction::Fraction;

#[macro_use]
mod fraction;

fn main() {
    let fr_one = Fraction::new(1, 2);
    println!("{}", fr_one);

    let fr_two = Fraction::new(3, 4);
    println!("{}", fr_two);

    let fr_sum = fr_one + fr_two;
    println!("{}", fr_sum);

    println!("{}", Fraction::new(1, 3));

    let fr_three = Fraction::from(10);
    println!("{}", fr_three);

    println!("{}", fr!(2,-6));

    println!("{}", fr!(3,4) / fr!(1,2) - fr!(3,2));
}
