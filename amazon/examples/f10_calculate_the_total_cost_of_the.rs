fn calculate_total_amount(data: String) -> i32 {
    if data.is_empty() {
        return 0;
    }

    let size = data.len();

    let mut current_data: i32 = 0;
    let mut last_data: i32 = 0;
    let mut total: i32 = 0;

    let mut symbol: char = '+';

    for i in 0..size {
        let current_char: char = data.chars().nth(i).unwrap();

        if current_char.is_digit(10) {
            current_data = (current_data * 10) + (current_char as i32 - '0' as i32);
        }

        if !current_char.is_digit(10) && current_char != ' ' || i == size - 1 {
            if symbol == '+' {
                total += last_data;
                last_data = current_data;
            } else if symbol == '-' {
                total += last_data;
                last_data = -current_data;
            } else if symbol == '*' {
                last_data = last_data * current_data;
            } else if symbol == '/' {
                last_data = last_data / current_data;
            }
            symbol = current_char;
            current_data = 0;
        }
    }
    total += last_data;
    return total;
}

fn main() {
    // driver code
    let cart_data = "2+3/7-1".to_string();
    let total_amount = calculate_total_amount(cart_data);
    println!("{}{:}", "Total amount of the cart is: ", total_amount);
}

// Time complexity = O(n)
// Space complexity = O(1)
