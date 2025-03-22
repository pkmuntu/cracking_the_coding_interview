use std::cmp;

fn merge_meetings(mut meeting_times: &mut Vec<Vec<i32>> , mut merged: &mut Vec<Vec<i32>>) {
   meeting_times.sort();

   for meeting in meeting_times.into_iter() {
       let size = merged.len();

       if size == 0 || merged[size-1][1] < meeting[0] {
          merged.push(meeting.to_vec());
       }else {
          merged[size - 1][1] = cmp::max(meeting[1], merged[size -1][1]);
       }
   }
}

fn main() {
  let mut meeting_times1: Vec<Vec<i32>> = vec![vec![1, 4], vec![2, 5], vec![6, 8], vec![7, 9], vec![10, 13]];
  let mut merged: Vec<Vec<i32>>= Vec::new();
  // First set of meetings
  merge_meetings(&mut meeting_times1,&mut merged);
  println!("{:?}",merged);
   merged.clear();
    // // Second set of meetings
  let mut meeting_times2: Vec<Vec<i32>> = vec![vec![4, 7], vec![1, 3], vec![8, 10], vec![2, 3], vec![6, 8]];
  merge_meetings(&mut meeting_times2,&mut merged);
  println!("{:?}",merged);  
}

// Time complexity = O(n * log())
// Space complexity = O(1)
