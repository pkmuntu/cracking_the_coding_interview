use std::cmp;
use std::collections::VecDeque;
fn mini_variation_length(nums: Vec<i32>, threshold: i32) -> i32 {
    let mut max_deque: VecDeque<i32> = VecDeque::new();
    let mut min_deque: VecDeque<i32> = VecDeque::new();
    let mut start = 0;
    let mut end = 0;
    let mut ans = 0;

    while end < nums.len() {
        // All elements greater than current index element gets removed from min_deque
        while !min_deque.is_empty() && nums[end] < nums[*min_deque.back().unwrap() as usize] {
            min_deque.pop_back(); // pop from end
        }
        // All elements smaller than current index element gets removed from min_deque
        while !max_deque.is_empty() && nums[end] > nums[*max_deque.front().unwrap() as usize] {
            max_deque.pop_back(); // pop from end
        }
        // append at end of both deques
        min_deque.push_back(end as i32);
        max_deque.push_back(end as i32);

        let variation =
            nums[*max_deque.front().unwrap() as usize] - nums[*min_deque.front().unwrap() as usize];
        if variation > threshold {
            start += 1;
            // A new sub-array is starting so elements from previous one should
            // be removed from both the deques
            if start > *min_deque.front().unwrap() {
                min_deque.pop_front();
            } // pop from front
            if start > *max_deque.front().unwrap() {
                max_deque.pop_front();
            } // pop from front
        }

        ans = cmp::max(ans, end - start as usize + 1);
        end += 1;
    }
    return ans as i32;
}

fn main() {
    // Driver code

    let traffic_rates: Vec<i32> = vec![10, 1, 2, 4, 7, 2];
    let threshold_mini_val: i32 = 5;
    println!(
        "{}{:}{}{:}{}",
        "The traffic of this customer changes by less than or equal to ",
        threshold_mini_val,
        " Gbps in a ",
        mini_variation_length(traffic_rates, threshold_mini_val),
        " day window\n"
    );
}

// Time complexity = O(n)
// Space complexity = O(n)
