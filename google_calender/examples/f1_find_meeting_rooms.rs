extern crate priority_queue;
use  priority_queue::DoublePriorityQueue;

fn min_meeting_rooms(mut meeting_times: &mut Vec<Vec<i32>>) -> i32 {
    if meeting_times.len() == 0 {
       return 0;
    }

    meeting_times.sort();
    let mut minheap = DoublePriorityQueue::new();
    minheap.push(meeting_times[0][1], meeting_times[0][1]);

    for i in 1..meeting_times.len() {
       let curr_start = meeting_times[i][0];
       let curr_end = meeting_times[i][1];
       let earliest_end = minheap.peek_min().unwrap();
       if earliest_end.0 <= &curr_start {
          minheap.pop_min();
       }

       // update the heap
       minheap.push(curr_end, curr_end);
    }
    return minheap.len() as i32;
}

fn main() {
  // Driver code
  let mut meeting_times: Vec<Vec<i32>> = vec![vec![2, 8], vec![3, 4], vec![3, 9], vec![5, 11], vec![8, 20], vec![11, 15]];
  println!("{:}",min_meeting_rooms(&mut meeting_times));
}

// Time complexity = O(n * log(n))
// Space complexity = O(n)


