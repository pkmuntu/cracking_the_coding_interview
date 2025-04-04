fn get_missing_id(arr: Vec<i32>, left: i32, right: i32, new_n: i32) -> i32 {
    if (left + 1) == right {
        return arr[left as usize] + new_n;
    }

    let middle = (left + right) / 2;

    let missing_nums = (arr[middle as usize] - arr[left as usize]) - (middle - left);
    if new_n > missing_nums {
        return get_missing_id(arr, middle, right, new_n - missing_nums);
    } else {
        return get_missing_id(arr, left, middle, new_n);
    }
}

fn resume_process(arr: Vec<i32>, n: i32) -> i32 {
    let pid = get_missing_id(arr.clone(), 0, arr.len() as i32, n);
    return pid;
}

fn main() {
    // Driver code

    let processes: Vec<i32> = vec![5, 7, 9, 10, 13];
    println!("{:}", resume_process(processes, 3));
}

// Time complexity = O(logn)
// Space complexity = O(1)
