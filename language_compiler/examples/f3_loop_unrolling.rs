fn loop_unrolling(code_block: String) -> String {
    let mut count_stack: Vec<i32> = Vec::new();
    let mut statement_stack: Vec<String> = Vec::new();
    let mut current_statement = String::from("");

    let mut n: i32 = 0;
    for i in 0..code_block.len() {
        let ch = code_block.chars().nth(i).unwrap();
        if ch.is_digit(10) {
            n = n * 10 + ch.to_digit(10).unwrap() as i32;
        } else if ch == '[' {
            // push the number n to count stack
            count_stack.push(n);
            statement_stack.push(current_statement);
            current_statement = "".to_string();
            n = 0;
        } else if ch == ']' {
            let mut unroll_statement: String = statement_stack.last().unwrap().to_string();
            // unroll current_n[current_statement] by appending current_statement n times
            let mut current_n: i32 = *count_stack.last().unwrap();
            count_stack.pop();
            while current_n > 0 {
                unroll_statement += &current_statement;
                current_n -= 1;
            }
            current_statement = unroll_statement;
        } else {
            current_statement.push(ch);
        }
    }
    return current_statement;
}

fn main() {
    let code_block: String = String::from("2[sum = sum + i; 2[i++; ]]");
    println!("{}", loop_unrolling(code_block));
}

// Time complexity = O(n*m)
// Space complexity = O(n + k)
