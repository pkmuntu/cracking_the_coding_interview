use std::collections::HashMap;
use std::collections::HashSet;

fn min_steps(k: Vec<i32>) -> i32 {
    let n = k.len();

    if n <= 1 {
        return 0;
    }

    let mut graph: HashMap<i32, Vec<i32>> = HashMap::new();
    for i in 0..n {
        if graph.contains_key(&k[i]) {
            graph.get_mut(&k[i]).unwrap().push(i as i32);
        } else {
            graph.entry(k[i]).or_insert(vec![i as i32]);
        }
    }

    let mut current: Vec<i32> = Vec::new();
    current.push(0);
    let mut visited: HashSet<i32> = HashSet::new();
    let mut step = 0;

    while !current.is_empty() {
        let mut next_node: Vec<i32> = Vec::new();

        for node in current.into_iter() {
            if node == (n - 1) as i32 {
                return step;
            }

            // check same value
            for child in graph.get(&k[node as usize]).unwrap().into_iter() {
                if !visited.contains(&*child) {
                    visited.insert(*child);
                    next_node.push(*child);
                }
            }

            // clear the list to prevent redundent search
            graph.get_mut(&k[node as usize]).unwrap().clear();

            // check meighbor
            if node + 1 < n as i32 && !visited.contains(&(node + 1)) {
                visited.insert(node + 1);
                next_node.push(node + 1);
            }

            if node - 1 >= 0 && !visited.contains(&(node - 1)) {
                visited.insert(node - 1);
                next_node.push(node - 1);
            }
        }
        current = next_node;
        step += 1;
    }
    return -1;
}

fn main() {
    // Driver code

    let k: Vec<i32> = vec![1, 2, 3, 4, 1, 3, 5, 3, 5];
    println!("{:}", min_steps(k));
}

// Time complexity = O(n)
// Space complexity = O(n)
