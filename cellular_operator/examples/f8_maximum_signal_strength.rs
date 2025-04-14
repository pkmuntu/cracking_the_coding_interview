fn max_signal_strength(stores: &Vec<i32>, mut k: i32) -> i32 {
    let mut left = 0;
    for right in 0..stores.len() {
        // If we included a zero in the window we reduce the value of k.
        // Since k is the maximum zeros allowed in a window
        if stores[right] == 0 {
            k -= 1;
        }

        // A negative k denots we have consumed all allowed flips and window has more
        // than allowed zeros, thus increment left pointer by 1 to keep the window size same.
        if k < 0 {
            // If the left element to be thrown out is zero we increase k.
            k += 1 - stores[left];
            left += 1;
        }
    }
    stores.len() as i32 - left as i32
}

fn main() {
    let stores = vec![1, 0, 0, 1, 1, 1, 0, 1, 0, 1];
    let mut key = 1;
    println!(
        "{}{}",
        "Maximum Contiguous Stores: ",
        max_signal_strength(&stores, key)
    );

    key = 3;
    println!(
        "{}{}",
        "Maximum Contiguous Stores: ",
        max_signal_strength(&stores, key)
    );
}

// Time complexiyt = O(n)
// Space complexity = o(1)
