fn recover_files(s: String) -> String {
    let mut stack: Vec<(i32, char)> = Vec::new();
    for i in 0..s.len() {
        if stack.len() > 0 && stack.last().unwrap().1 == 'C' && s.chars().nth(i).unwrap() == ')' {
            stack.pop();
        } else if s.chars().nth(i).unwrap() == '(' || s.chars().nth(i).unwrap() == ')' {
            stack.push((i as i32, s.chars().nth(i).unwrap()));
        }
    }

    let mut result: String = String::new();
    let mut count = 0;
    for x in (0..s.len()).rev() {
        if count <= stack.len() && stack.last().unwrap().0 == x as i32 {
            stack.pop();
            count += 1;
        } else {
            result.push(s.chars().nth(x).unwrap());
        }
    }
    return result.chars().rev().collect::<String>();
}

fn main() {
    // driver code
    println!("{}", recover_files("11)01(110)001(".to_string()));
}

// Time complexity = O(n)
// Space complexity = o(n)
