use std::cmp;
fn maximum_row_area(heights: Vec<i32>) -> i32 {
    let mut stack: Vec<i32> = Vec::new();

    // Initialize stack with the default l value
    stack.push(-1);

    let mut max_area = 0;
    for i in 0..heights.len() {
        while *stack.last().unwrap() != -1 && heights[*stack.last().unwrap() as usize] >= heights[i]
        {
            let top = stack.last().unwrap().clone();
            stack.pop();
            let height = heights[top as usize];
            let r = i as i32;
            let l = stack.last().unwrap();
            let width = (r - l) - 1;
            let area = height * width;

            max_area = cmp::max(max_area, area);
        }

        stack.push(i as i32);
    }

    // Case when no valid r value exists anymore
    while *stack.last().unwrap() != -1 {
        let top = stack.last().unwrap().clone();
        stack.pop();
        let height = heights[top as usize];
        let r = heights.len() as i32;
        let l = stack.last().unwrap();
        let width = (r - l) - 1;
        let area = height * width;

        max_area = cmp::max(max_area, area);
    }

    return max_area;
}

fn low_coverage(matrix: Vec<Vec<String>>) -> i32 {
    if matrix.len() == 0 {
        return 0;
    }
    let mut max_area = 0;
    let mut dp: Vec<i32> = vec![0; matrix[0].len()];

    for i in 0..matrix.len() {
        for j in 0..matrix[0].len() {
            // update the state of this row's dp array using the last row's dp array
            // by keeping track of the number of consecutive ones
            if matrix[i][j] == "1" {
                dp[j] = dp[j] + 1;
            } else {
                dp[j] = 0;
            }
        }
        // update max_area with the maximum area from this row's dp array
        max_area = cmp::max(max_area, maximum_row_area(dp.clone()));
    }
    return max_area;
}

fn main() {
    // Driver code

    let mall: Vec<Vec<String>> = vec![
        vec![
            "1".to_string(),
            "0".to_string(),
            "0".to_string(),
            "1".to_string(),
            "1".to_string(),
            "1".to_string(),
        ],
        vec![
            "1".to_string(),
            "0".to_string(),
            "1".to_string(),
            "1".to_string(),
            "0".to_string(),
            "1".to_string(),
        ],
        vec![
            "0".to_string(),
            "1".to_string(),
            "1".to_string(),
            "1".to_string(),
            "1".to_string(),
            "1".to_string(),
        ],
        vec![
            "0".to_string(),
            "0".to_string(),
            "1".to_string(),
            "1".to_string(),
            "1".to_string(),
            "1".to_string(),
        ],
    ];
    println!("{}{:}", "Maximum low coverage area is ", low_coverage(mall));
}

// Time complexixty = O(m*n)
// Space complexity = O(n)
