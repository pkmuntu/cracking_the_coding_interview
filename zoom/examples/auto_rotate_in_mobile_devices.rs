fn auto_rotate(matrix: &mut Vec<Vec<i32>>) {
    let mut left = 0;
    let mut right = matrix.len() - 1;

    while left < right {
        // `right - left` moves our squre in by 1 each outer iteration
        // the index is used to rotate the coordinates
        for i in 0..right - left {
            let top = left;
            let bottom = right;
            let top_left = matrix[top][left + i];

            // move bottomLeft to top_Left
            matrix[top][left + i] = matrix[bottom - i][left];

            // move bottomRight to bottomLeft
            matrix[bottom - i][left] = matrix[bottom][right - i];

            //move topRight to bottomRight
            matrix[bottom][right - i] = matrix[top + i][right];

            // set saved to topRight
            matrix[top + i][right] = top_left;
        }
        left += 1;
        right -= 1;
    }
    println!("{:?}", matrix);
}

fn main() {
    let mut matrix: Vec<Vec<i32>> = vec![
        vec![5, 1, 9, 11],
        vec![2, 4, 8, 10],
        vec![13, 3, 6, 7],
        vec![15, 14, 12, 16],
    ];

    auto_rotate(&mut matrix);
}

// Time complexity = O(n^2)
// Space complexity = O(1)
