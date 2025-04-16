fn merge_tweets(feed: &mut Vec<i32>, m: i32, tweets: Vec<i32>, n: i32) {
   let mut p1: i32 = m -1;
   let mut p2: i32 = n -1;

   for p in (-1..(m+n)).rev() {
      if p2 < 0 {
         break;
      }
      if p1 >= 0 && feed[p1 as usize] > tweets[p2 as usize] {
         feed[p as usize] = feed[p1 as usize];
         p1 -= 1;
      }else {
         feed[p as usize] = tweets[p2 as usize];
         p2 -= 1;
      }
   }
}
fn main(){
    let mut feed: Vec<i32> = vec![23, 33, 35, 41, 44, 47, 56, 91, 105, 0, 0, 0, 0, 0, 0];
    let m = 9;
    let tweets: Vec<i32> = vec![32, 49, 50, 51, 61, 99];
    let n = 6;
    merge_tweets(&mut feed, m, tweets, n);
    println!("{:?}",feed);
}

// Time complexity = O(n+m)
// Space complexity = O(1)

