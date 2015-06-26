fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    
    for i in 2..n {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn nth_prime(n: u64) -> u64 {
    let mut primes = vec![];
    for i in 2..n*n {
        if is_prime(i) {
            primes.push(i);
        }
    }
    primes[(n-1) as usize]
}

fn main() {
    const MAX: u64 = 10001;
    println!("{} prime: {}", MAX, nth_prime(MAX));
}
