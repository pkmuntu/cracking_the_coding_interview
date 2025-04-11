fn dfs(row: i32, col: i32, suffix: String, grid: &mut Vec<Vec<char>>) -> bool {
    if suffix.len() == 0 {
        return true;
    }

    //check if this is a valid cell
    if row < 0
        || row == 5
        || col < 0
        || col == 5
        || grid[row as usize][col as usize] != suffix.chars().nth(0).unwrap()
    {
        return false;
    }

    let mut ret = false;

    // mark the cell visisted
    grid[row as usize][col as usize] = '#';

    //explore the four neighboring directions
    let offsets: Vec<Vec<i32>> = vec![vec![0, 1], vec![1, 0], vec![0, -1], vec![-1, 0]];
    for offset in offsets.into_iter() {
        let row_offset = offset[0];
        let col_offset = offset[1];
        ret = dfs(
            row + row_offset,
            col + col_offset,
            suffix[1..].to_string(),
            grid,
        );
        if ret {
            break;
        }
    }

    // this will revert back the original value of the cell
    grid[row as usize][col as usize] = suffix.chars().nth(0).unwrap();
    return ret;
}

fn exists(mut grid: &mut Vec<Vec<char>>, word: String) -> bool {
    for row in 0..5 {
        for col in 0..5 {
            if dfs(row, col, word.clone(), &mut grid) {
                return true;
            }
        }
    }
    return false;
}

fn main() {
    let mut grid: Vec<Vec<char>> = vec![
        vec!['C', 'S', 'L', 'I', 'M'],
        vec!['O', 'I', 'L', 'M', 'O'],
        vec!['O', 'L', 'I', 'E', 'O'],
        vec!['R', 'T', 'A', 'S', 'N'],
        vec!['S', 'I', 'T', 'A', 'C'],
    ];
    let mut word: String = String::from("COIL");
    println!("{:?}", exists(&mut grid, word));

    word = "COCOON".to_string();
    println!("{:?}", exists(&mut grid, word));
}

// Time complexity = o(n * n^l)
// Sapce complexity O(l)
