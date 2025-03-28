fn peak_signal_strength(nums: Vec<i32>) -> i32 {
    let mut l = 0;
    let mut r = nums.len() - 1;
    while l < r {
        let mid = (l + r) / 2;
        if nums[mid] > nums[mid + 1] {
            r = mid;
        } else {
            l = mid + 1;
        }
    }
    return l as i32;
}

fn main() {
    // driver code

    // Example - 1
    let mut nums: Vec<i32> = vec![1, 2, 3, 4, 5];
    println!(
        "{:}{}",
        "The peak element is at index: ",
        peak_signal_strength(nums)
    );

    // Example - 2
    nums = vec![5, 4, 3, 2, 1];
    println!(
        "{:}{}",
        "The peak element is at index: ",
        peak_signal_strength(nums)
    );

    // Example - 3
    nums = vec![2, 3, 4, 5, 1];
    println!(
        "{:}{}",
        "The peak element is at index: ",
        peak_signal_strength(nums)
    );

    // Example - 4: Multiple Peaks
    nums = vec![1, 2, 4, 3, 6, 5, 10, 19, 8, 17];
    println!(
        "{:}{}",
        "The peak element is at index: ",
        peak_signal_strength(nums)
    );
}

// Time complexity = O(logn)
// Space complexity = O(1)
