fn process_log(s: String) -> i32 {
    let len = s.len();

    // sanity check if the length of the string is 0, simply return 0
    if s.len() == 0 || s.is_empty() {
        return 0;
    }

    let mut index = 0;

    // 1. check for white spaces
    // only the space character ' ' is considered a whitespaces character.
    while s.chars().nth(index).unwrap() == ' ' {
        index += 1;
    }

    // 2. check sing
    let mut is_negative = 0;
    if s.chars().nth(index).unwrap() == '-' {
        is_negative = 1;
        index += 1;
    } else if s.chars().nth(index).unwrap() == '+' {
        index += 1;
    }

    // 3. read until next non-digit character found
    let mut result = 0;
    for i in index..len {
        // check if the current character is a non-digit character is not
        if s.chars().nth(i).unwrap() >= '0' && s.chars().nth(i).unwrap() <= '9' {
            let char_to_digit_val = s.chars().nth(i).unwrap() as u32 - '0' as u32;

            // 4. check range (int underflow and overflow)
            if result > std::i32::MAX / 10
                || (result == std::i32::MAX / 10 && char_to_digit_val > 7)
            {
                if is_negative == 0 {
                    return std::i32::MAX;
                } else {
                    return std::i32::MIN;
                }
            }

            // adding digits at their desired place-value
            result = (result * 10) + char_to_digit_val as i32;
            index += 1;
        } else {
            break;
        }
    }
    if is_negative == 1 {
        return -result;
    } else {
        return result;
    }
}

fn main() {
    let s: Vec<String> = vec!["42", "    -123", "-42", "42 with words", "word 42"]
        .into_iter()
        .map(String::from)
        .collect();

    for i in 0..s.len() {
        let val = process_log(s[i].to_string());

        println!("{:}", val);
    }
}

// Time complexity = O(n)
// Space complexity = O(1)
