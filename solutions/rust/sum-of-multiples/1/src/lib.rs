pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    if factors.len() == 0 {
        return 0;
    }
    let mut ret = 0;
    let min = factors.iter().min().unwrap().clone();
    for i in min..limit {
        let mut is_multiple = false;
        for j in factors {
                if i % j == 0 {
                    is_multiple = true;
                }
            }
        if is_multiple {
            println!("{}", i);
            ret += i;
        }
    }
    ret
}