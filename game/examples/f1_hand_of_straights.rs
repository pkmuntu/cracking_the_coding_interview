use std::collections::HashMap;

fn is_hand_of_straights(hand: &mut Vec<i32>, k: i32) -> bool {
    if hand.len() % k as usize != 0 {
        return false;
    }

    let mut count: HashMap<i32, i32> = HashMap::new();
    for i in hand.into_iter() {
        if !count.contains_key(&i) {
            count.entry(*i).or_insert(1);
        } else {
            *count.get_mut(&i).unwrap() += 1;
        }
    }

    hand.sort();

    let mut i = 0;
    let n = hand.len();
    while i < n {
        let current = hand[i];
        for j in 0..k {
            if !count.contains_key(&(current + j)) || *count.get(&(current + j)).unwrap() == 0 {
                return false;
            }
            *count.get_mut(&(current + j)).unwrap() -= 1;
        }
        while i < n && *count.get(&hand[i]).unwrap() == 0 {
            i += 1;
        }
    }
    return true;
}

fn main() {
    let mut hand: Vec<i32> = vec![5, 2, 4, 4, 1, 3, 5, 6, 3];
    let k = 3;
    println!("{:}", is_hand_of_straights(&mut hand, k));

    let mut hand2: Vec<i32> = vec![1, 9, 3, 5, 7, 4, 2, 9, 11];
    let k = 2;
    println!("{:}", is_hand_of_straights(&mut hand2, k));
}

// Time complexity = O(nlog(n) + n*k)
// Space complexity = o(n)
