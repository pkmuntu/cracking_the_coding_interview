use std::cmp;
fn similarity_extent(sample1: String, sample2: String) -> i32 {
    let n = sample1.len();
    let m = sample2.len();

    // initialize an array to store all the computations
    let mut d: Vec<Vec<i32>> = vec![vec![0; m + 1]; n + 1];
    // cater the base case of the edit distance between
    // an empty string and non-empty string.
    for i in 0..n + 1 {
        d[i][0] = i as i32;
    }

    for j in 0..m + 1 {
        d[0][j] = j as i32;
    }

    // loop over the nucleotides both samples and compute the
    // edit distances D[i][j]'s.
    for i in 1..n + 1 {
        for j in 1..m + 1 {
            let left = d[i - 1][j]; // delete
            let down = d[i][j - 1]; // insert
            let left_down = d[i - 1][j - 1]; // replace

            if sample1.chars().nth(i - 1).unwrap() == sample2.chars().nth(j - 1).unwrap() {
                d[i][j] = 1 + cmp::min(left, cmp::min(down, left_down - 1));
            } else {
                d[i][j] = 1 + cmp::min(left, cmp::min(down, left_down));
            }
        }
    }
    return d[n][m];
}

//Driver code
fn main() {
    let sample1 = "abcdef".to_string();
    let sample2 = "azced".to_string();

    let operations = similarity_extent(sample1, sample2);

    println!("{:}", operations);
}

// Time complexity = O(n * m)
// Space complexity = O(n * m)
