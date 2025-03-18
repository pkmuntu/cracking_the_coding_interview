pub fn dfs(friends: &Vec<Vec<bool>>, n: i32, mut visited: &mut Vec<bool>, v: i32) {
    for i in 0..n {
        if !visited[i as usize] && friends[i as usize][v as usize] == true && i != v {
            visited[i as usize] = true;
            dfs(&friends, n, &mut visited, i);
        }
    }
}

pub fn friend_circles(friends: Vec<Vec<bool>>, n: i32) -> i32 {
    if n == 0 {
        return 0;
    }

    let mut visited: Vec<bool> = vec![false; n as usize];
    let mut num_circles: i32 = 0;
    for i in 0..n {
        if !visited[i as usize] {
            visited[i as usize] = true;
            dfs(&friends, n, &mut visited, i);
            num_circles += 1;
        }
    }
    return num_circles;
}

fn main() {
    let n = 4;
    let friends: Vec<Vec<bool>> = vec![
        vec![true, true, false, false],
        vec![true, true, true, false],
        vec![false, true, true, false],
        vec![false, false, false, true],
    ];
    let num = friend_circles(friends, n);
    println!("{}{:}", "Number of friends circles: ", num);
}

// Time complexity = O(n^2)
// Space complexity = O(n)
