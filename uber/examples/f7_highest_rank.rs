extern crate priority_queue;
use priority_queue::DoublePriorityQueue;

fn kth_highest_rank(rank: Vec<i32>, k: i32) -> i32 {
    let mut minheap = DoublePriorityQueue::new();

    // put first k element in minheap
    for i in 0..k {
        minheap.push(rank[i as usize], rank[i as usize]);
    }

    for i in k..rank.len() as i32 {
        if rank[i as usize] > *minheap.peek_min().unwrap().0 {
            minheap.pop_min();
            minheap.push(rank[i as usize], rank[i as usize]);
        }
    }

    return *minheap.peek_min().unwrap().0;
}

fn main() {
    // Driver code

    let driver_id: Vec<i32> = vec![1, 5, 12, 2, 11, 9, 7, 30, 20];
    let k = 3; // Supplied by a hidden API

    println!(
        "{}{:}{}",
        "Driver with the rank ",
        kth_highest_rank(driver_id, k),
        " is selected!"
    );
}

// Time complexity = o(n * logk)
// Space compleixty = o(k)
