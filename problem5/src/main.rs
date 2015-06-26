struct Generator {
    current: u64,
}

impl Iterator for Generator {
    type Item = u64;
    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current;
        self.current += 1;
        Some(current)
    }
}

fn generator() -> Generator {
    Generator { current: 11 }
}

fn main() {    
    let result = generator()
        .find(|&x| (2..21).all(|y| x % y == 0 ));
    let print = match result {
        Some(x) => x,
        None => panic!("Shit")
    };
    println!("{:?}", print);
}
