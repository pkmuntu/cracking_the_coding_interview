use std::cmp;
use std::collections::HashSet;

fn is_visited(visited: HashSet<String>, r: i32, c: i32) -> bool {
    // return whether the cell was visited or not
    let s = format!("{:}{}{:}", r, ",", c);
    return visited.contains(&s);
}

fn area(grid: Vec<Vec<i32>>, mut visited: &mut HashSet<String>, r: i32, c: i32) -> i32 {
    if r < 0
        || r >= grid.len() as i32
        || c < 0
        || c >= grid[0].len() as i32
        || is_visited(visited.clone(), r, c)
        || grid[r as usize][c as usize] == 0
    {
        return 0;
    }
    let s = format!("{:}{}{:}", r, ",", c);
    visited.insert(s);
    return 1
        + area(grid.clone(), &mut visited, r + 1, c)
        + area(grid.clone(), &mut visited, r - 1, c)
        + area(grid.clone(), &mut visited, r, c - 1)
        + area(grid.clone(), &mut visited, r, c + 1);
}

fn max_contiguous_area(grid: Vec<Vec<i32>>) -> i32 {
    let mut visited: HashSet<String> = HashSet::new();
    let mut ans = 0;
    for r in 0..grid.len() {
        for c in 0..grid[0].len() {
            ans = cmp::max(ans, area(grid.clone(), &mut visited, r as i32, c as i32))
        }
    }
    return ans;
}

fn main() {
    // driver code
    // Example - 1
    let mut grid: Vec<Vec<i32>> = vec![
        vec![0, 0, 1, 1, 0, 1],
        vec![0, 0, 1, 0, 0, 0],
        vec![1, 1, 1, 0, 1, 1],
        vec![0, 1, 0, 1, 0, 0],
        vec![1, 0, 1, 0, 0, 1],
    ];
    println!(
        "{}{:}",
        "Maximum Contiguous Area: ",
        max_contiguous_area(grid)
    );

    // Example - 2
    grid = vec![vec![0, 0, 0], vec![0, 0, 0], vec![0, 0, 0]];
    println!(
        "{}{:}",
        "Maximum Contiguous Area: ",
        max_contiguous_area(grid)
    );

    // Example - 3

    grid = vec![vec![1, 0, 1], vec![0, 0, 0], vec![1, 0, 1]];
    println!(
        "{}{:}",
        "Maximum Contiguous Area: ",
        max_contiguous_area(grid)
    );
}

// Time complexity = O(m * n)
// Space complexity = O(m * n)
