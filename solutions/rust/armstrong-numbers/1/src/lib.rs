pub fn is_armstrong_number(num: u32) -> bool {
    // used for a try at solving this without string conversion
    /*
    let mut mod_num = num;
    while mod_num / 10 != 0 {
        num_length += 1;
    }
    */
    let num_as_string = num.to_string();
    let mut sum: u32 = 0;

    for i in 0..num_as_string.len() {
        sum += num_as_string[i..i+1].parse::<u32>().unwrap().pow(num_as_string.len() as u32);
    }

    sum == num
}