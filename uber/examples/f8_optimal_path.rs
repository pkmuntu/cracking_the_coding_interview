use std::cmp;
fn min_path_sum(grid: &mut Vec<Vec<i32>>) -> i32 {
    if grid.len() == 0 || grid[0].len() == 0 {
        return -1;
    }
    let i = grid.len() - 1;
    let j = grid[0].len() - 1;

    for k in 0..i + 1 {
        for l in 0..j + 1 {
            if k > 0 && l > 0 {
                grid[k][l] = cmp::min(grid[k][l] + grid[k][l - 1], grid[k - 1][l] + grid[k][l]);
            } else if k > 0 || l > 0 {
                if l > 0 {
                    grid[k][l] += grid[k][l - 1];
                } else {
                    grid[k][l] += grid[k - 1][l];
                }
            }
        }
    }
    return grid[i][j];
}

fn main() {
    // Driver Code
    let mut grid: Vec<Vec<i32>> = vec![
        vec![5, 1, 9, 11],
        vec![11, 4, 8, 10],
        vec![13, 3, 6, 7],
        vec![5, 14, 12, 4],
    ];

    println!("{:}", min_path_sum(&mut grid));
}

// Time compleixty = O(nm)
// Space complexity = O(1)
