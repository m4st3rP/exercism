pub fn nth(n: u32) -> Option<u32> {
    if n > 0 {
        let mut prime_number = 0u32;
        let mut number = 2u32;
        let mut counter = 0u32;
        while counter < n {
            if is_prime(number) {
                counter += 1;
                prime_number = number;
            }
            number += 1;
        }
        Some(prime_number)
    } else {
        None
    }
}

fn is_prime(number: u32) -> bool {
    let max = number as f64;
    let max = max.sqrt() as u32;
    for i in 2..max+1 as u32 {
        if number % i == 0 {
            return false;
        }
    }
    true
}