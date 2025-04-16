fn dfs(graph: Vec<Vec<i32>>, color: &mut Vec<i32>, curr_color: i32, node: i32) -> bool {
    if color[node as usize] != 0 {
        return color[node as usize] == curr_color;
    }
    color[node as usize] = curr_color;
    for i in 0..graph[node as usize].len() {
        if !dfs(graph.clone(), color, -curr_color, graph[node as usize][i]) {
            return false;
        }
    }
    return true;
}

fn is_split_possible(graph: Vec<Vec<i32>>) -> bool {
    let mut color: Vec<i32> = vec![0; graph.len()];

    for i in 0..graph.len() {
        if color[i] == 0 && !dfs(graph.clone(), &mut color, 1, i as i32) {
            return false;
        }
    }
    return true;
}

fn main() {
    let graph: Vec<Vec<i32>> = vec![vec![3], vec![2, 4], vec![1], vec![0, 4], vec![1, 3]];
    println!("{:}", is_split_possible(graph));
}

// Time complexity = O(n+m)
// Space complexity = O(n)
