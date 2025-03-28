use std::cmp;
fn dfs(grid: Vec<Vec<i32>>, i: i32, j: i32, prev_val: i32, cache: &mut Vec<Vec<i32>>) -> i32 {
    if i < 0 || j < 0 || i > grid.len() as i32 - 1 || j > grid[0].len() as i32 - 1 {
        return 0;
    } else if prev_val > grid[i as usize][j as usize] {
        return 0;
    } else if cache[i as usize][j as usize] != 0 {
        return cache[i as usize][j as usize];
    }

    // Up
    let path_up = dfs(grid.clone(), i - 1, j, grid[i as usize][j as usize], cache);
    // Down
    let path_down = dfs(grid.clone(), i + 1, j, grid[i as usize][j as usize], cache);
    // Left
    let path_left = dfs(grid.clone(), i, j - 1, grid[i as usize][j as usize], cache);
    // Right
    let path_right = dfs(grid.clone(), i, j + 1, grid[i as usize][j as usize], cache);

    let max1 = cmp::max(path_up, path_down);
    let max2 = cmp::max(path_left, path_right);

    cache[i as usize][j as usize] = 1 + cmp::max(max1, max2);

    return cache[i as usize][j as usize];
}

fn maximum_routers(grid: Vec<Vec<i32>>) -> i32 {
    if grid.len() == 0 {
        return 0;
    }

    let mut res = 0;
    let mut cache: Vec<Vec<i32>> = vec![vec![0; grid[0].len()]; grid.len()];
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if cache[i][j] == 0 {
                let prev_val = grid[i][j];
                dfs(grid.clone(), i as i32, j as i32, prev_val, &mut cache);
                res = cmp::max(cache[i][j], res);
            }
        }
    }
    return res;
}

fn main() {
    // Driver code
    let grid: Vec<Vec<i32>> = vec![vec![2, 9, 6], vec![8, 4, 7], vec![5, 3, 1]];
    println!("{}{:}", "Maximum Routers are ", maximum_routers(grid));
}

// Time complexity = O(m * n)
// Space complexity = O(m * n)
