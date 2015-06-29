extern crate num;

struct Collatz {
    i: u64
}

impl Collatz {
    fn new(starting_point: u64) -> Collatz {
        Collatz { i: starting_point }
    }
}

impl Iterator for Collatz {
    type Item = u64;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.i == 0 {
            return None;
        }

        let current = self.i;
        
        self.i = match self.i {
            1 => 0,
            _ => match self.i & 1 {
                0 => self.i / 2,
                _ => 3 * self.i + 1
            }
        };
        
        Some(current)
    }
}

fn main() {
    let max = 1_000_000; 
    let mut c: Vec<Vec<u64>> = (1..max).map(|x| Collatz::new(x).collect()).collect();
    c.sort_by(|a, b| b.len().cmp(&a.len()));
    println!("The Collatz Sequence with the longest sequence under {} is {} with sequence length of: {}", max, c[0][0], c[0].len());
}