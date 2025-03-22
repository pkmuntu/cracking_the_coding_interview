use std::cmp;
use std::collections::HashMap;

fn two_sets_of_days(hours_per_day: Vec<i32>, k: i32) -> i32 {
    let mut hmap: HashMap<i32, i32> = HashMap::new();
    let mut sum: i32 = 0;
    let mut lsize = std::i32::MAX;
    let mut result = std::i32::MAX;
    hmap.entry(0).or_insert(-1);
    for i in 0..hours_per_day.len() {
        sum += hours_per_day[i];
        hmap.entry(sum).or_insert(i as i32);
    }

    sum = 0;
    for i in 0..hours_per_day.len() {
        sum += hours_per_day[i];
        if hmap.contains_key(&(sum - k)) {
            // stors minimum length of sub-array ending with index <= i with sum k.
            lsize = cmp::min(lsize, i as i32 - hmap[&(sum - k)]);
        }

        // search for any sub-array starting with index i + 1 with sum k.
        if hmap.contains_key(&(sum + k)) && lsize < std::i32::MAX {
            // updates the result only if both left and right sub-array exists.
            result = cmp::min(result, hmap[&(sum + k)] - i as i32 + lsize);
        }
    }
    if result == std::i32::MAX {
        return -1;
    } else {
        return result;
    }
}

fn main() {
    let hours_per_day: Vec<i32> = vec![1, 2, 2, 3, 2, 6, 7, 2, 1, 4, 8];
    let k = 5;
    println!("{:}", two_sets_of_days(hours_per_day, k));
}

// Time complexity = O(n)
// Space complexity = O(n)
