fn is_palindrome(number: u32) -> bool {
    let mut temp = number;
    let mut result = 0u32;
    while temp > 0 {
        result = result * 10 + temp % 10;
        temp /= 10;
    }
    result == number
}

fn main() {        
    let (mut max_i, mut max_j, mut max_product) = (0, 0, 0);
    let (min, max): (u32, u32) = (100, 1000); 
    for i in min..max {
        for j in min..max {
            let product = i * j;
            let palindrome = is_palindrome(product);            
            if palindrome && product > max_product {
                max_i = i;
                max_j = j;
                max_product = product;
            }
       }
    }
    
    println!("{} * {} = {}", max_i, max_j, max_product);
    println!("Done");
}