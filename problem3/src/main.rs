fn _largest_prime_easy(n: u64) -> u64 { // Takes WAY too much memory
    match (1..n+1).filter(|&x| n % x == 0).max() {
        Some(x) => x,
        None => panic!("Uh oh")
    }
}

fn largest_prime_factor(mut n: u64) -> u64 {
    let mut factors: Vec<u64> = vec![];
    let mut d = 2;
    while n > 1 {
        while n % d == 0 {
            factors.push(d);
            n /= d
        }
        d+= 1;
        if d*d > n {
            if n > 1 {
                factors.push(n);
            }
            break;
        }
    }
    
    match factors.iter().max() {
        Some(x) => *x,
        None => panic!("Shit")
    }
}

fn main() {
    println!("{}", largest_prime_factor(600_851_475_143));
}
