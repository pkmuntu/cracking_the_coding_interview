use std::collections::VecDeque;
pub struct TweetsPredictor {
    pub size: i32,
    pub queue: VecDeque<i32>,
    pub len_queue: i32,
    pub window_sum: i32,
}

impl TweetsPredictor {
    pub fn new(s: i32) -> Self {
        TweetsPredictor {
            size: s,
            queue: VecDeque::new(),
            len_queue: 0,
            window_sum: 0,
        }
    }
    pub fn predict_tweets(&mut self, val: i32) -> f64 {
        let pop_val;
        if self.len_queue == self.size {
            pop_val = *self.queue.front().unwrap() as i32;
            self.queue.pop_front();
        } else {
            self.len_queue = self.len_queue + 1;
            pop_val = 0;
        }

        self.queue.push_back(val);

        self.window_sum = self.window_sum + val - pop_val;
        let s: f64 = self.window_sum as f64 / self.len_queue as f64;
        return s;
    }
}

fn main() {
    // Driver code
    let values: Vec<i32> = vec![1, 10, 3, 5];
    let mut count_tweets: TweetsPredictor = TweetsPredictor::new(3);

    for i in values.into_iter() {
        println!("{:?}", count_tweets.predict_tweets(i));
    }
}

// Time complexity = O(1)
// Space complexity = O(m)
