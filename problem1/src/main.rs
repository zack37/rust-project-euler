fn is_multiple(test: u32) -> bool {
    test % 3 == 0 || test % 5 == 0
}

fn main() {
    let sum = (1u32..1000).filter(|&x| is_multiple(x))
        .fold(0u32, |acc, cur| acc + cur);
    
    println!("{}", sum);
}
