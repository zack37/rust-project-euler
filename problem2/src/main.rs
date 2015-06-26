struct Fibonacci {
    cur: u64,
    next: u64   
}

impl Iterator for Fibonacci {
    type Item = u64;
    fn next(&mut self) -> Option<u64> {
        let cur = self.cur;
        let new_next = self.cur + self.next;
        
        self.cur = self.next;
        self.next = new_next;                
    
        Some(cur)
    }
}

fn fibonacci() -> Fibonacci {
    Fibonacci { cur: 0, next: 1}
}

fn main() {
    let evens: Vec<u64> = fibonacci()
        .take_while(|&x| x <= 4_000_000u64)
        .filter(|&x| x % 2 == 0)
        .collect();
        
    println!("{:?}", evens);
}
