use std::collections::HashMap;
use std::collections::HashSet;
fn valid_sequence(priorities: Vec<i32>) -> bool {
    let mut map: HashMap<i32, HashSet<i32>> = HashMap::new();

    for i in 0..priorities.len() {
        map.insert(priorities[i], HashSet::new());
    }
    map.get_mut(&0).unwrap().insert(0);
    for i in 0..priorities.len() {
        for k in map.clone().get(&priorities[i]).unwrap().iter() {
            for increment in k - 1..k + 2 {
                if increment > 0 && map.contains_key(&(priorities[i] + increment)) {
                    map.get_mut(&(priorities[i] + increment))
                        .unwrap()
                        .insert(increment);
                }
            }
        }
    }
    return map.get(&(priorities[priorities.len() - 1])).unwrap().len() > 0;
}

fn main() {
    println!("{:}", valid_sequence(vec![0, 1, 3, 5, 8, 12, 17]));
}

// Time complexity = O(n^2)
// Space complexity = O(n^2)
