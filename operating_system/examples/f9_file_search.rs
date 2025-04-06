fn find_files(p: String) -> Vec<String> {
    let files: Vec<String> = vec![
        "data".to_string(),
        "dataaa".to_string(),
        "data2".to_string(),
    ];
    let mut result: Vec<String> = Vec::new();
    for s in files.into_iter() {
        if is_match(s.to_string(), p.to_string()) {
            result.push(s);
        }
    }
    return result;
}

fn is_match(s: String, p: String) -> bool {
    let m = s.len();
    let n = p.len();
    let mut dp: Vec<Vec<bool>> = vec![vec![false; n + 1]; m + 1];
    dp[0][0] = true;

    for i in 1..m + 1 {
        let s_index = i - 1;
        for j in 1..n + 1 {
            let p_index = j - 1;
            if p.chars().nth(p_index).unwrap() == '*' {
                if i > 0
                    && (p.chars().nth(p_index - 1).unwrap() == s.chars().nth(s_index).unwrap()
                        || p.chars().nth(p_index - 1).unwrap() == '.')
                {
                    dp[i][j] = dp[i][j - 2] || dp[i - 1][j];
                } else {
                    dp[i][j] = dp[i][j - 2];
                }
            } else if i > 0 && p.chars().nth(p_index).unwrap() == '.' {
                dp[i][j] = dp[i - 1][j - 1];
            } else if i > 0 && p.chars().nth(p_index).unwrap() == s.chars().nth(s_index).unwrap() {
                dp[i][j] = dp[i - 1][j - 1];
            }
        }
    }
    return dp[m][n];
}

fn main() {
    println!("{:?}", find_files("data*".to_string()));
    println!("{:?}", find_files("data.".to_string()));
}

// Time complexity = O(m*n)
// Space complexity = o(m*n)
