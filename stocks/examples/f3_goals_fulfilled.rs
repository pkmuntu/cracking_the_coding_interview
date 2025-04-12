use std::collections::HashMap;
fn goals_fulfilled(trades: Vec<i32>) -> bool {
    let mut frequency_map: HashMap<i32, i32> = HashMap::new();
    let mut imagined_map: HashMap<i32, i32> = HashMap::new();

    for n in trades.clone().into_iter() {
        if frequency_map.contains_key(&(n)) {
            *frequency_map.get_mut(&n).unwrap() += 1;
        } else {
            frequency_map.entry(n).or_insert(1);
        }
    }

    for n in trades.clone().into_iter() {
        if frequency_map[&n] == 0 {
            continue;
        } else if imagined_map.contains_key(&n) {
            if imagined_map[&n] > 0 {
                *imagined_map.get_mut(&n).unwrap() -= 1;
                *imagined_map.get_mut(&(n + 1)).unwrap() += 1;
            }
        } else if frequency_map.get(&(n + 1)).unwrap() > &0
            && frequency_map.get(&(n + 2)).unwrap() > &0
        {
            *frequency_map.get_mut(&(n + 1)).unwrap() -= 1;
            *frequency_map.get_mut(&(n + 2)).unwrap() -= 1;
            if imagined_map.contains_key(&(n + 3)) {
                *imagined_map.get_mut(&(n + 3)).unwrap() += 1;
            } else {
                imagined_map.entry(n + 3).or_insert(1);
            }
        } else {
            return false;
        }
        *frequency_map.get_mut(&n).unwrap() = -1;
    }
    return true;
}

fn main() {
    // Driver code

    let trades: Vec<i32> = vec![1, 2, 3, 3, 4, 4, 5, 5];

    if !goals_fulfilled(trades) {
        println!("The goals have not been fulfilled!");
    } else {
        println!("The goals have been fulfilled!");
    }
}

// Time complexity = O(n)
// Space complexity = O(n)
