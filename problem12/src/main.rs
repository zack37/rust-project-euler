struct TriangleNumbers {
    i: u64,
    last_sum: u64
}

impl TriangleNumbers {
    fn new() -> TriangleNumbers {
        TriangleNumbers { i: 1, last_sum: 1 }
    }
}

impl Iterator for TriangleNumbers {
    type Item = u64;
    fn next(&mut self) -> Option<Self::Item> {
        self.i += 1;
        self.last_sum += self.i;
        Some(self.last_sum)
    }
}

fn main() {
    //let functional = TriangleNumbers::new().find(|&x| _fun_factors(x).len() > 100).unwrap();
    //println!("functional: {:?}", functional);
    let performant = TriangleNumbers::new().find(|&x| factors(x).len() > 500).unwrap();    
    println!("performant: {:?}", performant);
}

fn _fun_factors(max: u64) -> Vec<u64>  {
    (1..max+1).filter(move |&x| max % x == 0).collect()
}

fn factors(max: u64) -> Vec<u64> {
    let mut facs = vec![1, max];
    for i in 2..(max as f64).sqrt() as u64 {
        if max % i == 0 {
            facs.push(i);
            facs.push(max / i);
        }
    }
    
    facs
    
}