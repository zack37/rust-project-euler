extern crate num;

use num::{BigUint};
use num::bigint::{ToBigUint};

fn main() {
    let base = 2.to_biguint().unwrap();
    let pow = pow(&base, 1000);
    let as_string = format!("{}", pow);
    let chars: Vec<char> = as_string.chars().collect();
    let sum = chars.iter().map(|x| x.to_digit(10).unwrap()).fold(0, |acc, cur| acc + cur);
    println!("sum: {}", sum);
}

fn pow(base: &BigUint, exp: usize) -> BigUint {
    (1..exp+1).fold(1.to_biguint().unwrap(), |acc, _| acc * base)
}
