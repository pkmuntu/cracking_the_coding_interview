use std::cmp;
use std::collections::HashMap;
use std::collections::VecDeque;
fn find_dictionary(messages: Vec<String>) -> String {
    // Step 0: Create data structures and find all unique letters.
    let mut adj_list: HashMap<char, Vec<char>> = HashMap::new();
    let mut counts: HashMap<char, i32> = HashMap::new();

    for message in messages.clone().into_iter() {
        for c in message.chars() {
            counts.entry(c).or_insert(0);
            adj_list.entry(c).or_insert(vec![]);
        }
    }

    // Step 1: Find all edges.
    for i in 0..messages.len() - 1 {
        let message1 = messages[i].to_string();
        let message2 = messages[i + 1].to_string();
        // Check that message2 is not a prefix of message1.
        if message1.len() > message2.len() && message1.find(&message2) == Some(0) {
            return "".to_string();
        }
        // Find the first non match and insert the corresponding relation.
        for j in 0..cmp::min(message1.len(), message2.len()) {
            if message1.chars().nth(j).unwrap() != message2.chars().nth(j).unwrap() {
                adj_list
                    .get_mut(&message1.chars().nth(j).unwrap())
                    .unwrap()
                    .push(message2.chars().nth(j).unwrap());
                *counts.get_mut(&message2.chars().nth(j).unwrap()).unwrap() += 1;
                break;
            }
        }
    }

    // Step 2: Breadth-first search.
    let mut result = "".to_string();
    let mut queue: VecDeque<char> = VecDeque::new();
    for (itr, val) in counts.iter() {
        let c: char = *itr;
        if *val == 0 {
            queue.push_back(c);
        }
    }

    while !queue.is_empty() {
        let c = queue.pop_front().unwrap();
        result.push(c);
        for next in adj_list.get(&c).unwrap().into_iter() {
            *counts.get_mut(next).unwrap() -= 1;
            if *counts.get(next).unwrap() == 0 {
                queue.push_back(*next);
            }
        }
    }

    if result.len() < counts.len() {
        return "".to_string();
    }
    return result;
}

fn main() {
    // Example - 1
    let mut messages: Vec<String> = vec![
        "mzosr", "mqov", "xxsvq", "xazv", "xazau", "xaqu", "suvzu", "suvxq", "suam", "suax", "rom",
        "rwx", "rwv",
    ]
    .into_iter()
    .map(String::from)
    .collect();
    println!("{}{}", "Dictionary = ", find_dictionary(messages));

    // Example - 2
    messages = vec![
        "vanilla", "alpine", "algor", "port", "norm", "nylon", "ophellia", "hidden",
    ]
    .into_iter()
    .map(String::from)
    .collect();
    println!("{}{}", "Dictionary = ", find_dictionary(messages));
}

// Time complexity = O(c) c: total length of all the messages in the input list
// Space complexity = O(1)
