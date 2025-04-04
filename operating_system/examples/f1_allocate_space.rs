use std::collections::HashMap;
fn allocate_space(processes: Vec<i32>, new_p: i32) -> i32 {
    let mut count = 0;
    let mut sum = 0;
    let mut res: HashMap<i32, i32> = HashMap::new();
    res.entry(0).or_insert(1);
    for i in 0..processes.len() {
        sum += processes[i];
        if res.contains_key(&(sum - new_p)) {
            count += res.get(&(sum - new_p)).unwrap();
        }
        if !res.contains_key(&sum) {
            res.entry(sum).or_insert(1);
        } else {
            *res.get_mut(&sum).unwrap() += 1;
        }
    }
    return count;
}

fn main() {
    // Driver code
    let processes: Vec<i32> = vec![
        1, 2, 3, 4, 5, 6, 7, 1, 23, 21, 3, 1, 2, 1, 1, 1, 1, 1, 12, 2, 3, 2, 3, 2, 2,
    ];
    println!("{:}", allocate_space(processes, 1));
}

// Time complexity = O(n)
// Space complexity = O(n)
