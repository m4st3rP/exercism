pub fn reply(message: &str) -> &str {
    // first check if message is empty
    if message.to_string().trim().is_empty() {
        return "Fine. Be that way!";
    }

    let mut has_no_alphabetic = true;
    for c in message.chars() {
        if c.is_alphabetic() {
            has_no_alphabetic = false;
        }
    }
    let last_char = message.to_string().trim().chars().last().unwrap();

    // check first because gets detected as capslock when only digits
    if has_no_alphabetic && last_char != '?' {
        return "Whatever.";
    } else if has_no_alphabetic && last_char == '?' {
        return "Sure.";
    }

    let is_capslock = message.to_uppercase() == message;
    if is_capslock && last_char != '?' {
        "Whoa, chill out!"
    } else if is_capslock && last_char == '?' {
        "Calm down, I know what I'm doing!"
    } else if !is_capslock && last_char == '?' {
        "Sure."
    } else {
        "Whatever."
    }
}