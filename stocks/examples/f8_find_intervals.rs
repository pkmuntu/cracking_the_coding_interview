fn find_intervals(prices: Vec<i32>) -> Vec<i32> {
    // number of predicted prices in the time window
    let n = prices.len();

    let mut intervals: Vec<i32> = vec![0; n];

    // initialize a stack to store the indices of the intervals
    let mut stack: Vec<i32> = Vec::new();

    // iterate over the prices
    for curr_inter in 0..n {
        // current intervals predicted price
        let current_price = prices[curr_inter];

        // check if the stack is empty or not and also
        // check if the price at current interval is higher than
        // the interval's price at top of the stack
        while !stack.is_empty() && prices[*stack.last().unwrap() as usize] < current_price {
            let prev_inter = stack.pop().unwrap() as usize;
            intervals[prev_inter] = (curr_inter - prev_inter) as i32;
        }

        // push the current index onto the stack
        stack.push(curr_inter as i32);
    }

    return intervals;
}

fn main() {
    // Driver code
    let prices: Vec<i32> = vec![68, 71, 78, 67, 66, 69, 79, 68];

    let intervals = find_intervals(prices);
    println!("{}{:?}", "Minimum intervals: ", intervals);
}

// Time complexity = O(n)
// Space complexity = O(n)
