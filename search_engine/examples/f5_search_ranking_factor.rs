fn search_ranking(page_scores: Vec<i32>) -> Vec<i32> {
  let length = page_scores.len();
  let mut ranking: Vec<i32> = vec![0; page_scores.len()];

  ranking[0] = 1;
  for i in 1..length{
    ranking[i] = page_scores[i-1] * ranking[i-1];
  }
  let mut right = 1;
  for i in (0..length).rev() {
    ranking[i] = ranking[i] * right;
    right = right * page_scores[i];
  }
  return ranking;
}

fn main() {
  let page_scores: Vec<i32> = vec![3, 5, 1, 1, 6, 7, 2, 3, 4, 1, 2];
  let ranking: Vec<i32> = search_ranking(page_scores);
  println!("{:?}", ranking);
}

// Time complexity = O(n)
// Space complexity = O(n)
