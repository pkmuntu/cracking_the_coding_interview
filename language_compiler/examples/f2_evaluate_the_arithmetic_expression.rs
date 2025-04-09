fn evaluate_expression(expression: String) -> i32 {
    let mut number: i32 = 0;
    let mut sign: i32 = 1;
    let mut output: i32 = 0;
    let mut stack: Vec<i32> = Vec::new();
    for i in 0..expression.len() {
        let c = expression.chars().nth(i).unwrap();
        if c.is_digit(10) {
            number = number * 10 + c.to_digit(10).unwrap() as i32;
        } else if c == '-' || c == '+' {
            output += number * sign;
            if c == '-' {
                sign = -1;
            } else {
                sign = 1;
            }
            number = 0;
        } else if c == '(' {
            stack.push(output);
            stack.push(sign);
            output = 0;
            sign = 1;
        } else if c == ')' {
            output += sign * number;
            output *= stack.last().unwrap();
            stack.pop();
            output += stack.last().unwrap();
            stack.pop();
            number = 0;
        }
    }
    return output + number * sign;
}

fn main() {
    let expression = String::from("{4 - (10 + 52) + 99}");
    println!("{:}", evaluate_expression(expression));
}

// Time complexity = O(n)
// Space complexity = O(n)
