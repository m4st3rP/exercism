const BASE: u64 = 2;

pub fn square(s: u32) -> u64 {
    assert!(0 < s && s <= 64, "Square must be between 1 and 64");
    BASE.pow(s-1)
}

pub fn total() -> u64 {
    let mut ret = 0;
    for i in 0..64 {
        ret += BASE.pow(i);
    }
    ret
}