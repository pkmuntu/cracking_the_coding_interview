use std::cmp;
use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::FromIterator;
fn densest_deployment(coordinates: Vec<Vec<i32>>) -> i32 {
    let mut res = std::i32::MAX;
    let mut x: HashMap<i32, HashSet<i32>> = HashMap::new();
    for p in coordinates.clone().into_iter() {
        let mut set: HashSet<i32> = HashSet::new();
        if !x.contains_key(&p[0]) {
            set.insert(p[1]);
            x.entry(p[0]).or_insert(set);
        } else {
            set.insert(p[1]);
            x.get_mut(&p[0]).unwrap().insert(p[1]);
        }
    }
    for i in 0..coordinates.clone().len() {
        for j in i + 1..coordinates.clone().len() {
            let p = coordinates[i][0];
            let p2 = coordinates[j][0];
            if p == p2 {
                continue;
            }
            if x.get(&p).unwrap().len() < 2 || x.get(&p2).unwrap().len() < 2 {
                continue;
            }
            let s = x.get(&p).unwrap().intersection(x.get(&p2).unwrap());
            let y: Vec<&i32> = Vec::from_iter(s);
            for k in 1..y.len() {
                res = cmp::min(res, (p2 - p) * (y[k] - y[k - 1])).abs();
            }
        }
    }
    if res == std::i32::MAX {
        return 0;
    } else {
        return res;
    }
}

fn main() {
    // driver code

    // Example - 1
    let mut coordinates: Vec<Vec<i32>> =
        vec![vec![0, 3], vec![2, 3], vec![1, 2], vec![0, 1], vec![2, 1]];
    println!(
        "{}{:}",
        "Densest Deployment: ",
        densest_deployment(coordinates)
    );

    // Example - 2
    coordinates = vec![
        vec![0, 1],
        vec![0, 3],
        vec![3, 3],
        vec![3, 1],
        vec![5, 1],
        vec![5, 3],
    ];
    println!(
        "{}{:}",
        "Densest Deployment: ",
        densest_deployment(coordinates)
    );

    // Example - 3
    coordinates = vec![vec![1, 0], vec![1, 3], vec![3, 3], vec![3, 0]];
    println!(
        "{}{:}",
        "Densest Deployment: ",
        densest_deployment(coordinates)
    );
    // Example - 4
    coordinates = vec![vec![0, 0], vec![1, 3], vec![3, 3], vec![3, 0]];
    println!(
        "{}{:}",
        "Densest Deployment: ",
        densest_deployment(coordinates)
    );
}

// Time complexity = O(n^2)
// Space complexity = O(n)
