use std::collections::HashMap;
use std::collections::VecDeque;
fn schedule_process(vertices: i32, edges: Vec<Vec<i32>>, sorted_order: &mut Vec<i32>) {
    if vertices <= 0 {
        return;
    }

    // a. Initialize the graph
    let mut in_degree: HashMap<i32, i32> = HashMap::new();
    let mut graph: HashMap<i32, Vec<i32>> = HashMap::new();
    for i in 0..vertices {
        in_degree.entry(i).or_insert(0);
        graph.entry(i).or_insert([].to_vec());
    }

    // b. Build the graph
    for i in 0..edges.len() {
        let parent = edges[i][1];
        let child = edges[i][0];
        graph.get_mut(&parent).unwrap().push(child); // put the child into it's parent's list
        *in_degree.get_mut(&child).unwrap() += 1; // increment child's in_degree
    }

    // c. Find all sources i.e., all vertices with 0 in-degrees
    let mut sources = VecDeque::new();
    for i in 0..in_degree.len() {
        if *in_degree.get(&(i as i32)).unwrap() == 0 {
            sources.push_back(i);
        }
    }

    // // d. For each source, add it to the sorted_order and subtract one from all of its children's in-degrees
    // // if a child's in-degree becomes zero, add it to the sources queue
    while !sources.is_empty() {
        let vertex = sources.front().unwrap().clone();
        sources.pop_front();
        sorted_order.push(vertex as i32);
        let children: Vec<i32> = graph.get(&(vertex as i32)).unwrap().to_vec(); // get the node's children to decrement their in-degrees
        for child in children.into_iter() {
            *in_degree.get_mut(&child).unwrap() -= 1;
            if *in_degree.get(&(child)).unwrap() == 0 {
                sources.push_back(child as usize);
            }
        }
    }

    if sorted_order.len() != vertices as usize {
        // topological sort is not possible as the graph has a cycle
        sorted_order.clear();
    }
}

fn main() {
    // Driver code
    let dependencies: Vec<Vec<i32>> = vec![vec![1, 0], vec![2, 0], vec![3, 1], vec![3, 2]];
    let mut result: Vec<i32> = Vec::new();
    schedule_process(4, dependencies, &mut result);
    println!("{}{:?}", "Topological sort: ", result);
}

// Time complexity = O(V + E)
// Space complexity = O(V + E)
