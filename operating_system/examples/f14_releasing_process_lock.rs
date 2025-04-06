fn find_unreleased_lock(process: Vec<i32>) -> i32 {
    let mut lo = 0;
    let mut hi = process.len() - 1;
    while lo < hi {
        let mut mid = lo + (hi - lo) / 2;
        if mid % 2 == 1 {
            mid -= 1;
        }
        if process[mid] == process[mid + 1] {
            lo = mid + 2;
        } else {
            hi = mid;
        }
    }
    return process[lo];
}
fn main() {
    println!(
        "{:}",
        find_unreleased_lock(vec![1, 1, 2, 2, 3, 3, 4, 4, 5, 8, 8])
    );
}

// Time complexity = O(logn)
// Space complexit = O(1)
