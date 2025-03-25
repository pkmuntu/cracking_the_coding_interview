use std::collections::VecDeque;
const OPEN_SPACE: i32 = std::i32::MAX;
const DROP_POINT: i32 = 0;

fn warehouse_and_drop_points(warehouse: &mut Vec<Vec<i32>>) {
    // First of all check if the given warehouse is empty or not
    let m = warehouse.len() as i32;
    if m == 0 {
        return;
    }

    // Save the warehouse row and col sizes in m and n
    let n = warehouse[0].len() as i32;

    // Initialize a queue to implement a Bread-First search approach
    let mut q: VecDeque<(i32, i32)> = VecDeque::new();

    // Filling the queue with drop points row, col indexes of a warehouse
    for row in 0..m {
        for col in 0..n {
            if warehouse[row as usize][col as usize] == DROP_POINT {
                q.push_back((row, col));
            }
        }
    }

    let directions: Vec<Vec<i32>> = vec![vec![1, 0], vec![-1, 0], vec![0, 1], vec![0, -1]];

    // Filling the open space with the directions to nearest drop point using the queeu and BFS
    // approach
    while !q.is_empty() {
        let point = q.pop_front().unwrap();
        let row = point.0;
        let col = point.1;

        for direction in directions.clone().into_iter() {
            let r = row + direction[0];
            let c = col + direction[1];

            if r < 0 || c < 0 || r >= m || c >= n || warehouse[r as usize][c as usize] != OPEN_SPACE
            {
                continue;
            }
            warehouse[r as usize][c as usize] = warehouse[row as usize][col as usize] + 1;
            q.push_back((r, c));
        }
    }
}

fn main() {
    // driver code
    let mut warehouse: Vec<Vec<i32>> = vec![
        vec![2147483647, -1, 0, 2147483647],
        vec![2147483647, 2147483647, 2147483647, -1],
        vec![2147483647, -1, 2147483647, -1],
        vec![0, -1, 2147483647, 2147483647],
    ];
    // ToString, a utility function to print the vector
    println!("{}{:?}", "Given Warehouse: ", warehouse);
    warehouse_and_drop_points(&mut warehouse);
    println!("{}{:?}", "Filled Warehouse: ", warehouse);
}

// Time complexity = O(nm)
// Space complexity = O(mn)
