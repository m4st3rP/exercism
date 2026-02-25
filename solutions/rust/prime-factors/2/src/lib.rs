pub fn factors(n: u64) -> Vec<u64> {
    let mut current_val = n;
    let mut ret = Vec::new();

    let mut i = 2;
    while current_val > 1 {
        while current_val % i == 0 {
            ret.push(i);
            current_val /= i;
        }
        i += 1;
    }
    ret
}