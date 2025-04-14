fn max_users(arr: Vec<i32>, size: i32, k: i32) -> Vec<i32> {
    let mut mqueue: Vec<i32> = Vec::new(); // Store given arr indices.
    let mut result: Vec<i32> = Vec::new(); // Store maximum value of every window slide.

    for i in 0..size {
        while mqueue.len() > 0 && arr[mqueue[mqueue.len() - 1] as usize] <= arr[i as usize] {
            mqueue.pop();
        }

        mqueue.push(i);
        if mqueue[0] == i - k {
            mqueue.remove(*mqueue.first().unwrap() as usize);
        }

        // If the given array index is greater than and equal to the window slide size,
        // then push the maximum value in result arr.
        if i >= k - 1 {
            result.push(arr[mqueue[0] as usize]);
        }
    }

    return result;
}

fn main() {
    // Driver code
    let arr: Vec<i32> = vec![4, 7, 12, 16, 8, 3, 13, 20, 5, 9, 22, 2];
    let size = arr.len();
    let k = 7;

    let result = max_users(arr, size as i32, k);

    println!("{:?}", result);
}

// Time complexity = O(n)
// Space complexity = O(k)
