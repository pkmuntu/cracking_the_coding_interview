fn is_palindrome(arr: Vec<i32>, mut left: i32, mut right: i32) -> bool {
    while left < right {
        if arr[left as usize] != arr[right as usize] {
            return false;
        }
        left += 1;
        right -= 1;
    }
    return true;
}

fn transmission_error(arr: Vec<i32>) -> i32 {
    // Initialize left and right at each end
    let mut left: i32 = 0;
    let mut right: i32 = arr.len() as i32 - 1;

    while left < right {
        // Clause if element at both ends are same
        if arr[left as usize] == arr[right as usize] {
            left += 1;
            right -= 1;
        } else {
            // check if ignoring left gives a palindromic sequence
            if is_palindrome(arr.clone(), left + 1, right) {
                return 1;
            }

            // Check if ignoring right gives a palindromic sequence
            if is_palindrome(arr.clone(), left, right - 1) {
                return 1;
            }

            // Multiple routers exist
            return -1;
        }
    }
    // No diversion router was used
    return 0;
}

fn main() {
    let arr: Vec<i32> = vec![1, 2, 3, 3, 4, 2, 1];
    let res = transmission_error(arr);
    if res == 1 || res == 0 {
        println!("Network Sustained. No Transmission Error Occurred!\n");
    } else {
        println!("Network Broke. Transmission Error Occurred!\n");
    }
}

// Time complexity = O(n)
// Space complexity = O(1)
