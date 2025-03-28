use std::cmp;
use std::collections::HashMap;
use std::collections::VecDeque;

fn total_time(main_server_id: i32, parents: Vec<i32>, delays: Vec<i32>) -> i32 {
    let n = parents.len();
    if n <= 1 {
        return 0;
    }

    let mut children: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut res = 0;

    for index in 0..parents.len() {
        let val = parents[index];
        let mut temp_list: Vec<i32> = Vec::new();
        if children.contains_key(&val) {
            temp_list = children.get(&val).unwrap().to_vec();
        }
        temp_list.push(index as i32);
        children.entry(val).or_insert(temp_list);
    }

    let mut queue: VecDeque<(i32, i32)> = VecDeque::new();
    queue.push_front((main_server_id, delays[main_server_id as usize]));

    while !queue.is_empty() {
        let node = queue.pop_back().unwrap();
        let curr_id = node.0;
        let curr_time = node.1;

        res = cmp::max(res, curr_time);
        let mut temp: Vec<i32> = Vec::new();
        if children.contains_key(&curr_id) {
            temp = children.get(&curr_id).unwrap().to_vec();
        }

        for child in temp.into_iter() {
            queue.push_back((child, curr_time + delays[child as usize]));
        }
    }
    return res;
}

fn main() {
    // Driver code

    let main_server_id = 0;
    let parents: Vec<i32> = vec![-1, 0, 0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6];
    let delays: Vec<i32> = vec![1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0];

    println!(
        "{}{:}{}",
        "Time required by message to reach all devices is ",
        total_time(main_server_id, parents, delays),
        " units"
    );
}

// Time complexity = O(n)
// Space complexity = O(n)
