fn main() {
    let sum_of_squares: u64 = (1..101).map(|x| x * x).fold(0u64, |acc, cur| acc + cur);
    let sum: u64 = (1..101).fold(0u64, |acc, cur| acc + cur);
    let answer = (sum * sum) - sum_of_squares;
    println!("(1 + 2 + ... + 100)^2 - 1^2 + 2^2 + ... + 100^2");
    println!("answer: {}", answer);
}