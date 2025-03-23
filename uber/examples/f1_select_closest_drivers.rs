use priority_queue::PriorityQueue;

#[derive(Default, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub struct Location {
    x: i32,
    y: i32,
}

impl Location {
    fn new(x: i32, y: i32) -> Self {
        Location { x: x, y: y }
    }

    fn dist_from_origin(&mut self) -> i32 {
        //ignoring sqrt
        return (self.x * self.x) + (self.y * self.y);
    }
}

fn find_closest_drivers(location: Vec<Location>, k: i32, res: &mut Vec<Location>) {
    let mut maxheap = PriorityQueue::new();
    // first put `k` location to maxheap
    for i in 0..k {
        maxheap.push(location[i as usize].clone(), location[i as usize].clone());
    }

    for i in k as usize..location.len() {
        let top = maxheap.peek_mut().unwrap().0;
        if location[i].clone().dist_from_origin() < top.dist_from_origin() {
            maxheap.peek();
            maxheap.pop();
            maxheap.push(location[i as usize].clone(), location[i as usize].clone());
        }
    }

    while !maxheap.is_empty() {
        res.push(maxheap.peek().unwrap().0.clone());
        maxheap.pop();
    }
}

fn main() {
    // Driver Code
    let locations: Vec<Location> = vec![
        Location::new(1, 3),
        Location::new(3, 4),
        Location::new(2, -1),
    ];
    let mut result: Vec<Location> = Vec::new();
    find_closest_drivers(locations, 2, &mut result);
    print!("Here are the k drivers locations closest to the user in the Seattle area: ");
    for p in result.into_iter() {
        print!("{}{:?}{}{:?}{}", "[", p.x, ", ", p.y, "] ");
    }
}

// Time complexity = O(nlog(k))
// Space complexity = O(k)
