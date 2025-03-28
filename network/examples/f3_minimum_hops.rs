use std::cmp;
fn minimum_hops(values: Vec<i32>) -> i32 {
    if values.len() < 2 {
        return 0;
    }
    let mut max_reach = values[0];
    let mut curr_reach = values[0];
    let mut hops = 1;
    for i in 0..values.len() as i32 {
        if curr_reach < i {
            hops = hops + 1;
            curr_reach = max_reach;
        }
        max_reach = cmp::max(max_reach, values[i as usize] + i);
    }
    return hops;
}

fn main() {
    // Driver code
    let switch_array: Vec<i32> = vec![4, 1, 1, 3, 1, 1, 1];
    println!(
        "{}{:}",
        "Minimum hops to final router are: ",
        minimum_hops(switch_array)
    );
}

// Time complexity = O(n)
// Space complexity = O(n)
