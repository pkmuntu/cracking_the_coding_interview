use std::collections::VecDeque;

fn update_configuration(grid: &mut Vec<Vec<i32>>) -> i32 {
    let mut queue: VecDeque<(i32, i32)> = VecDeque::new();
    // Step 1). build the inital set of udpated routers
    let rows = grid.len();
    let cols = grid[0].len();
    for r in 0..rows {
        for c in 0..cols {
            if grid[r][c] == 2 {
                queue.push_front((r as i32, c as i32));
            }
        }
    }

    // Mark the round / level, _i.e_ the ticker of timestamp
    queue.push_front((-1, -1));
    // Step 2). start the transmitting process via BFS
    let mut minutes_elapsed = -1;
    // Four Neighbors, up, right, down and left
    let directions: Vec<Vec<i32>> = vec![vec![-1, 0], vec![0, 1], vec![1, 0], vec![0, -1]];

    while !queue.is_empty() {
        let p = queue.pop_back().unwrap();
        let row = p.0;
        let col = p.1;
        if row == -1 {
            // we finish one round of processing
            minutes_elapsed += 1;
            if !queue.is_empty() {
                queue.push_front((-1, -1));
            }
        } else {
            // this is an updated router
            // then it would transmit the update to its neighbors
            for d in directions.clone().into_iter() {
                let neighbor_row = row + d[0];
                let neighbor_col = col + d[1];
                if neighbor_row >= 0
                    && neighbor_row < rows as i32
                    && neighbor_col >= 0
                    && neighbor_col < cols as i32
                {
                    if grid[neighbor_row as usize][neighbor_col as usize] == 1 {
                        grid[neighbor_row as usize][neighbor_col as usize] = 2;
                        queue.push_front((neighbor_row, neighbor_col));
                    }
                }
            }
        }
    }
    return minutes_elapsed;
}

fn main() {
    // Driver Code
    let mut grid: Vec<Vec<i32>> = vec![
        vec![1, 1, 0, 0, 1],
        vec![0, 1, 0, 1, 1],
        vec![1, 1, 2, 0, 1],
        vec![1, 0, 1, 1, 1],
        vec![1, 1, 0, 1, 1],
    ];
    println!("{:}", update_configuration(&mut grid));
}

// Time complexity = O(n)
// Space complexity = O(n)
