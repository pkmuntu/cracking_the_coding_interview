fn determine_location(region: Vec<Vec<i32>>, loss_value: i32) -> Vec<i32> {
    let mut row = region.len() - 1;
    let mut col = 0;

    while row >= 0 && col < region[0].len() {
        if region[row][col] > loss_value {
            row -= 1;
        } else if region[row][col] < loss_value {
            col += 1;
        } else {
            let temp = vec![row as i32, col as i32];
            return temp;
        }
    }
    return vec![-1, -1];
}

fn main() {
    // Driver code
    let region: Vec<Vec<i32>> = vec![
        vec![1, 4, 7, 11, 15],
        vec![2, 5, 8, 12, 19],
        vec![3, 6, 9, 16, 22],
        vec![10, 13, 14, 17, 24],
        vec![18, 21, 23, 26, 30],
    ];
    let loss_value = 5;
    let res = determine_location(region, loss_value);
    println!(
        "{}{:?}",
        "The coordinates of the loss value region are: ", res
    );
}

// Time complexity = O(M + N)
// Space complexity = O(1)
