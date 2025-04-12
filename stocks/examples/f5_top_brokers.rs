extern crate priority_queue;
use priority_queue::DoublePriorityQueue;
use std::collections::HashMap;

fn top_brokers(broker_ids: Vec<i32>, k: i32) -> Vec<i32> {
    // find the frequency of each number
    let mut num_frequency_map: HashMap<i32, i32> = HashMap::new();
    for n in broker_ids.into_iter() {
        if !num_frequency_map.contains_key(&n) {
            num_frequency_map.entry(n).or_insert(1);
        } else {
            *num_frequency_map.get_mut(&n).unwrap() += 1;
        }
    }

    let mut minheap = DoublePriorityQueue::new();
    // go through all numbers of the num_frequency_map and push then in the minHeap,
    // which will have top k frequent numbers. If the heap size is more than k,
    // whe remove the smallest (top) number
    for (first, second) in num_frequency_map.iter() {
        minheap.push(first, second);
        if minheap.len() as i32 > k {
            minheap.pop_min();
        }
    }

    // create a list of  top k numbers
    let mut top_numbers: Vec<i32> = Vec::new();

    while !minheap.is_empty() {
        let pair = minheap.pop_min().unwrap();
        top_numbers.push(*pair.0);
    }
    return top_numbers;
}

fn main() {
    // Driver code

    let result: Vec<i32> = top_brokers(vec![1, 3, 5, 12, 11, 12, 11, 12, 5], 3);
    println!(
        "{}{:?}",
        "Here are the IDs of the top K brokers of the quarter: ", result
    );
}

// Time complexity = O(n * logk)
// Space complexity = O(n)
