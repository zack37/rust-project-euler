fn main() {
    let (min, max) = (1, 999); // Trial and error to find minimum range    
    
    let (a, b, c) = (min..max).flat_map(|a| -> Vec<(usize, usize, usize)> {
        (min..max).flat_map(|b| -> Vec<(usize, usize, usize)> {
            (min..max).map(|c| (a, b, c)).collect()
        }).collect()
    })
    .find(|&(a, b, c)| a < b && b < c
        && (a*a) + (b*b) == (c*c)
        && a + b + c == 1000).unwrap();
    
    println!("a({}), b({}), c({})", a, b, c);
    
//    let result: Vec<(usize, usize, usize)> = (min..max)
//        .flat_map(|a| -> Vec<(usize, usize, usize)> {
//            (min..max).flat_map(|b| -> Vec<(usize, usize, usize)> {
//                (min..max).map(|c| {
//                    (a, b, c)
//                })
//                .filter(|&(a, b, c)| a < b && b < c
//                    && (a*a) + (b*b) == (c*c)
//                    && a + b + c ==  1000)
//                    .collect()
//            }).collect()
//        }).collect();
//    println!("{:?}", result[0]);
}

/*
    const int MinRange = 200;
    const int Elements = 226;
   (
        from a in Enumerable.Range(MinRange, Elements)
        from b in Enumerable.Range(MinRange, Elements)
        from c in Enumerable.Range(MinRange, Elements)
        where a < b && b < c
            && a.Sqr() + b.Sqr() == c.Sqr()
            && a + b + c == 1000
        select new {a, b, c}
    )
    .Single()
*/
