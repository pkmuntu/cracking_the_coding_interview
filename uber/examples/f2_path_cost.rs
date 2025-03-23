use std::cmp;

// This method calculate the amount of water trapped. The only argument passed in the
// elevation map, in from of an array.
fn path_cost(elevation_map: Vec<i32>) -> i32 {
    let mut water = 0; // Keep track fo the total water as we traverse the elevation map

    let n = elevation_map.len();

    // lsit to store the left_max and right max of each point in the map
    let mut left_max: Vec<i32> = vec![0; n];
    let mut right_max: Vec<i32> = vec![0; n];

    // default values
    left_max[0] = elevation_map[0];
    right_max[n - 1] = elevation_map[n - 1];

    // filling teh left_max list:
    for i in 1..n {
        left_max[i] = cmp::max(left_max[i - 1], elevation_map[i]);
    }

    // filling the right_max list
    let mut c = n - 2;
    while c > 0 {
        right_max[c] = cmp::max(right_max[c + 1], elevation_map[c]);
        c = c - 1;
    }

    right_max[c] = cmp::max(right_max[c + 1], elevation_map[c]);

    // calculating the amount of water
    for i in 0..n {
        water += cmp::min(left_max[i], right_max[i]) - elevation_map[i];
    }
    return water;
}

fn main() {
    // Driver code

    let elevation_map: Vec<i32> = vec![1, 2, 1, 3, 1, 2, 1, 4, 1, 0, 0, 2, 1, 4];

    println!("{}{:}", "Accumulated water: ", path_cost(elevation_map));
}

// Time complexity = O(n)
// Space complexity = O(n)
