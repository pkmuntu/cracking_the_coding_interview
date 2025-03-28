fn dfs(matrix: &mut Vec<Vec<i32>>, r: i32, c: i32, curr_id: i32, new_id: i32) {
    // chekc bounds of the matrix
    if r < 0
        || c < 0
        || r >= matrix.len() as i32
        || c >= matrix[0].len() as i32
        || matrix[r as usize][c as usize] != curr_id
    {
        return;
    }

    matrix[r as usize][c as usize] = new_id;
    dfs(matrix, r - 1, c, curr_id, new_id);
    dfs(matrix, r + 1, c, curr_id, new_id);
    dfs(matrix, r, c - 1, curr_id, new_id);
    dfs(matrix, r, c + 1, curr_id, new_id);
}

fn update_vlan(matrix: &mut Vec<Vec<i32>>, r: i32, c: i32, new_id: i32) -> Vec<Vec<i32>> {
    let curr_id = matrix[r as usize][c as usize];
    if curr_id == new_id {
        return matrix.clone();
    }
    dfs(matrix, r, c, curr_id, new_id);
    return matrix.clone();
}

fn main() {
    // Drive code

    let mut matrix: Vec<Vec<i32>> = vec![
        vec![1, 1, 1, 1, 1],
        vec![1, 1, 1, 1, 0],
        vec![1, 1, 1, 0, 0],
        vec![1, 1, 0, 1, 0],
        vec![1, 1, 0, 0, 1],
    ];
    let r = 1;
    let c = 1;
    let new_id = 2;
    let res = update_vlan(&mut matrix, r, c, new_id);
    println!("{}{:?}", "Swtches with Updated VLAN IDs:\n", res);
}

// Time complexity = O(n)
// Space complexity = O(n)
