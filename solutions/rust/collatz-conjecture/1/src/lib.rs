pub fn collatz(n: u64) -> Option<u64> {
    let mut n = n;
    let mut counter = 0;

    if n < 1 {
        return None;
    }
    while n > 1 {
        if n % 2 == 0 {
            n /= 2;
        } else {
            n = n * 3 + 1;
        }
        counter += 1;
    }
    Some(counter)
}