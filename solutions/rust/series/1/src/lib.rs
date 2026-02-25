pub fn series(digits: &str, len: usize) -> Vec<String> {
    let mut ret = Vec::new();
    if digits.len() < len {
        return ret;
    }
    for i in 0..digits.len()-len+1 {
       ret.push(digits[i..i+len].to_string());
    }
    ret
}