use std::collections::HashMap;
fn check_delivery(packages: Vec<i32>, k: i32) -> bool {
    let mut curr_sum: i32 = 0;
    let mut map: HashMap<i32, i32> = HashMap::new();
    map.entry(0).or_insert(-1);
    for i in 0..packages.len() {
        curr_sum += packages[i];

        if k != 0 {
            curr_sum = curr_sum % k;
        }

        if map.contains_key(&curr_sum) {
            if i as i32 - map[&curr_sum] > 1 {
                return true;
            }
        } else {
            map.entry(curr_sum).or_insert(i as i32);
        }
    }
    return false;
}

fn main() {
    let packages: Vec<i32> = vec![11, 42, 54, 44, 49, 26];
    let k = 10;
    println!("{:}", check_delivery(packages, k));
}

// Time complexity = O(n)
// Space complexity = O(min(n, k))
