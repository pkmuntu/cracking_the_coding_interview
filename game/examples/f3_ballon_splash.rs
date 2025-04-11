fn shoot_balloons(strr: String, k: i32) {
    let mut sb = strr;
    let mut stack: Vec<i32> = Vec::new();
    let mut i = 0;
    while i < sb.len() {
        if i == 0 || sb.chars().nth(i) != sb.chars().nth(i - 1) {
            stack.push(1);
        } else {
            let incremented = stack.pop().unwrap() + 1;
            // if the character count reaches k, pop it from the stack
            if incremented == k {
                let mut c = 0;
                while c < i + 1 - (i - k as usize + 1) {
                    sb.remove(i - k as usize + 1);
                    c += 1;
                }
                i = i - k as usize;
            } else {
                stack.push(incremented);
            }
        }
        i += 1;
    }
    println!("{}", sb.to_string());
}

fn main() {
    shoot_balloons("sammmaddkkkdasa".to_string(), 3);
}

// Time complexity = O(n)
// Space complexity = O(n)
