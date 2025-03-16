use std::{cmp::Ordering, collections::BinaryHeap};

#[derive(Eq, PartialEq)]
struct Reverse {
    age: i32,
}

impl PartialOrd for Reverse {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.age.partial_cmp(&self.age)
    }
}

impl Ord for Reverse {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

struct MedianOfAges {
    large_list: BinaryHeap<Reverse>, // containing second half of number
    small_list: BinaryHeap<i32>,     // containing first half of number
}

impl MedianOfAges {
    // initialze your data structure here.
    fn new() -> Self {
        Self {
            large_list: BinaryHeap::new(),
            small_list: BinaryHeap::new(),
        }
    }

    fn insert_age(&mut self, age: i32) {
        if self.large_list.is_empty() || age > self.large_list.peek().unwrap().age {
            self.large_list.push(Reverse { age: age });
        } else {
            self.small_list.push(age);
        }

        // check for heappify
        // either large_list have greater element or both have same element
        if self.large_list.len() > self.small_list.len() + 1 {
            self.small_list.push(self.large_list.pop().unwrap().age);
        } else if self.large_list.len() < self.small_list.len() {
            self.large_list.push(Reverse {
                age: self.small_list.pop().unwrap(),
            });
        }
    }

    fn find_median(&self) -> f64 {
        if self.large_list.len() == self.small_list.len() {
            // return the sum of two value div by 2.0
            return (self.large_list.peek().unwrap().age as f64
                + *self.small_list.peek().unwrap() as f64)
                / 2.0;
        }
        return self.large_list.peek().unwrap().age as f64;
    }
}

fn main() {
    // Driver code
    let mut obj = MedianOfAges::new();
    obj.insert_age(22);
    obj.insert_age(35);
    println!(
        "{}{:}",
        "The recommended content will be for ages under: ",
        obj.find_median()
    );
    obj.insert_age(30);
    println!(
        "{}{:}",
        "The recommended content will be for ages under: ",
        obj.find_median()
    );
    obj.insert_age(25);
    println!(
        "{}{:}",
        "The recommended content will be for ages under: ",
        obj.find_median()
    );
}

//Time complexity(Insert age) = O(logn)
//Time complexity(Find median) = O(1)
//Space complexity = O(n)
