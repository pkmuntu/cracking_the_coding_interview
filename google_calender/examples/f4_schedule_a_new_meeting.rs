use std::cmp;

fn insert_meeting(meeting_times: &mut Vec<Vec<i32>>, new_meeting: Vec<i32>, output: &mut Vec<Vec<i32>>) {
  meeting_times.sort();

  let mut i = 0;
  let n = meeting_times.len();

  while i < n && new_meeting[0] > meeting_times[i][0] {
     output.push(meeting_times[i].to_vec());
     i = i + 1;
  }
  let size = output.len();
  if size == 0 || output[size-1][1] < new_meeting[0] {
      output.push(new_meeting.to_vec());
  }else {
      output[size-1][1] = cmp::max(output[size-1][1], new_meeting[1]);
  }

  while i < n {
     if output[size-1][1] < meeting_times[i][0] {
         output.push(meeting_times[i].to_vec());
     }else{
         output[size-1][1]=cmp::max(output[size-1][1], meeting_times[i][1]);
     }
     i = i + 1;
  }
}
fn main() {
  let mut meeting_times: Vec<Vec<i32>> = vec![vec![1, 3], vec![4, 6], vec![8, 10], vec![10, 12], vec![13, 15], vec![16, 18]];
  let mut output: Vec<Vec<i32>>= Vec::new();
  let new_meeting: Vec<i32>= vec![9, 13];
  insert_meeting(&mut meeting_times, new_meeting, &mut output);
  println!("{:?}",output);
}


// Time complexity = O(n)
// Space complexity = O(n)
