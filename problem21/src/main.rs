fn main() { 
    let amicable_pairs: Vec<u64> = (2..10_000).filter_map(amicable_pair).collect();
    let ap_sum = amicable_pairs.iter().fold(0, |acc, cur| acc + cur);
    println!("Amicable pairs under 10,000: {:?} and their sum: {}", amicable_pairs, ap_sum);
}

fn amicable_pair(start: u64) -> Option<u64> {    
    let divisors_sum = sum_of_divisors(start);
    let divisors_sum_sum = sum_of_divisors(divisors_sum);
    
    if divisors_sum_sum == start && start != divisors_sum {
        return Some(start);
    }
    
    None
}

fn sum_of_divisors(n: u64) -> u64 {
    (1..n).filter(|x| n % x == 0).fold(0, |acc, cur| acc + cur)
}