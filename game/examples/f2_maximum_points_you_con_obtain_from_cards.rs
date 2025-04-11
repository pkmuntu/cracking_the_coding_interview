use std::cmp;

fn max_points(deck: Vec<i32>, k: i32) -> i32 {
    let mut left: i32 = 0;
    let mut right: i32 = deck.len() as i32 - k;
    let mut total: i32 = 0;
    let mut best: i32;

    for i in right..deck.len() as i32 {
        total += deck[i as usize];
    }
    best = total;

    for _i in 0..k {
        total += deck[left as usize] - deck[right as usize];
        best = cmp::max(best, total);
        left += 1;
        right += 1;
    }
    return best;
}

fn main() {
    let deck: Vec<i32> = vec![5, 3, 4, 4, 2, 3, 2, 6, 3];
    let k = 4;
    println!("{:}", max_points(deck, k));
}

// Time complexity = O(k)
// Space complexity = O(1)
