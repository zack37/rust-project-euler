fn main() {
    let max = 200_000_000;
    let primes = sieve_of_atkin(max);
    println!("Done finding primes");
    let sum = primes.iter().fold(0, |acc, cur| acc + cur); 
    println!("Sum of primes up to {}: {}", max, sum);
}

fn sieve_of_atkin(limit: usize) -> Vec<usize> {
    let mut is_prime = vec![false; limit+1];
    let sqrt = (limit as f64).sqrt();
    let i_limit = limit as isize;
    
    for x in 1..sqrt as isize {
        let xx: isize = x*x;
        for y in 1..sqrt as isize {
            let yy = y*y;
            let mut n = 4 * xx + yy;
            if n <= i_limit && (n % 12 == 1 || n % 12 == 5) {
                is_prime[n as usize] ^= true;
            }
            
            n = 3 * xx + yy;
            if n <= i_limit && n%12 == 7 {
                is_prime[n as usize] ^= true;
            }
            
            n = 3 * xx - yy;
            if x > y && n <= i_limit && n % 12 == 11 {
                is_prime[n as usize] ^= true;
            }
        }
    }
    
    let mut primes = vec![2, 3];
    
    for n in (5..(sqrt+1.0) as usize) {
        if is_prime[n] {
            primes.push(n);
            let nn = n*n;
            let mut k = nn;
            while k <= limit {
                is_prime[k] = false;
                k += nn;
            }
        }
    }
    
    for n in (sqrt+1.0) as usize..limit+1 {
        if is_prime[n] {
            primes.push(n);
        }
    }
    
    primes
}
