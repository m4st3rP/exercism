const TARGET_SUM: u32 = 1000;
const SEARCH_LIMIT: u32 = TARGET_SUM/2;

pub fn find() -> Option<u32> {
    for a in 1..SEARCH_LIMIT {
        for b in 1..SEARCH_LIMIT {
            let c = TARGET_SUM - a - b;
            if is_specific_triplet(a, b, c, TARGET_SUM) {
                println!("a:{} b:{} c:{} sum:{}", a, b,c, a*b*c);
                return Some(a*b*c);
            }
        }
    }
    None
}

fn is_specific_triplet(a: u32, b: u32, c: u32, sum: u32) -> bool {
    a.pow(2) + b.pow(2) == c.pow(2) && a + b + c == sum
}

// wanted to see what a, b and c are
pub fn main() {
    find();
}