extern crate num;

use num::{BigUint};
use num::bigint::{ToBigUint};

fn main() {
    let fac = fac(100);
    let as_string = format!("{}", fac);
    let chars: Vec<char> = as_string.chars().collect();
    let sum = chars.iter().map(|x| x.to_digit(10).unwrap()).fold(0, |acc, cur| acc + cur);
    println!("sum: {}", sum);
}

fn fac(base: usize) -> BigUint {
    (1..base+1).fold(1.to_biguint().unwrap(), |acc, cur| acc * cur.to_biguint().unwrap())
}
