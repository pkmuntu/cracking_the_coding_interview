extern crate priority_queue;
use priority_queue::PriorityQueue;
use std::collections::HashMap;

fn reorganize_results(initial_order: String) -> String {
  let mut map: HashMap<char, i32> = HashMap::new();
  for c in initial_order.chars() {
    let freq;
    if !map.contains_key(&c) {
        freq = 1;
    }else {
        freq = map[&c] + 1;
    }
    if freq > ((initial_order.len() + 1) / 2 ) as i32 {
        return initial_order;
    }
    if !map.contains_key(&c) {
        map.entry(c).or_insert(freq);
    }else {
        *map.get_mut(&c).unwrap() = freq;
    }
  }

  let mut pq = PriorityQueue::new();
  for (key, val) in map.iter() {
     pq.push(key, *val);
  }
  let mut result = "".to_string();
  while !pq.is_empty() {
    let first = pq.pop().unwrap();
    if result.len() == 0 || *first.0 != result.chars().nth(result.len() - 1).unwrap()    {
       result.push(*first.0);
       if first.1 - 1 > 0 {
          pq.push(first.clone().0,first.clone().1 -1);
       }
    }
    else{
       let second = pq.pop();
       result.push(*second.unwrap().0);
       if second.unwrap().1-1 > 0 {
          pq.push(first.clone().0, first.clone().1);
       }
       pq.push(first.clone().0, first.clone().1);
    }
  }
  return result;
}

fn main() {
  let initial_order = "bbbnncc".to_string();
  println!("{}", reorganize_results(initial_order));
}

// Time complexity = O(n)
// Space complexity = O(1)
