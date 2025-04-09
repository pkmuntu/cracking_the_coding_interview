fn first_failing_step(n: i32) -> i32 {
    let mut first = 1;
    let mut last = n;
    while first < last {
        let mid = first + (last - first) / 2;
        if is_failing_step(mid) {
            // Calling the API to determine if the step fails
            last = mid;
        } else {
            first = mid + 1;
        }
    }
    return first;
}

fn main() {
    let n = 40; // Number of steps
    let s = 28; // Failing step
    set_failing_step(s);
    println!("{:}", first_failing_step(n));
}

// Time complexity = O(log(n))
// Space complexity = O(1)
