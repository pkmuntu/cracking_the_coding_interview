fn milestone_reached(matrix: Vec<Vec<i32>>, milestone: i32) -> Vec<i32> {
    let mut failed: Vec<i32> = vec![-1, 1];

    let m = matrix.len();
    if m == 0 {
        return failed;
    }

    let n = matrix[0].len();

    // binary search
    let mut left = 0;
    let mut right = m * n - 1;
    let mut middle_idx;
    let mut middle_element;

    while left <= right {
        middle_idx = (left + right) / 2;
        middle_element = matrix[middle_idx / n][middle_idx % n];
        if milestone == middle_element {
            failed = vec![(middle_idx / n) as i32, (middle_idx % n) as i32];
            return failed;
        } else {
            if milestone < middle_element {
                right = middle_idx - 1;
            } else {
                left = middle_idx + 1;
            }
        }
    }
    return vec![-1, -1];
}

fn main() {
    // Driver code

    let matrix: Vec<Vec<i32>> = vec![
        vec![0, 2, 4, 6, 8],
        vec![10, 12, 14, 18, 22],
        vec![24, 30, 34, 60, 64],
    ];
    let milestone = 24;
    let res = milestone_reached(matrix, milestone);
    println!(
        "{}{:}{}{:}",
        "Milestone reached on day ",
        res[1] + 1,
        " of week ",
        res[0] + 1
    );
}

// Time complexity = O(long(mn))
// Space complexiy = O(1)
