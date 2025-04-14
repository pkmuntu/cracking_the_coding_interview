use std::collections::HashSet;
use std::collections::VecDeque;

fn neighbors(src: String) -> Vec<String> {
    let mut res: Vec<String> = Vec::new();
    for (i, ch) in src.chars().enumerate() {
        // calculate the new state by moving the current number in opposite direction
        let mut num = ch.to_digit(10).unwrap() as i32;
        if num == 0 {
            num = 9;
        } else {
            num = num - 0 - 1;
        }
        let s = format!(
            "{}{}{}",
            src[0..i].to_string(),
            num,
            src[i + 1..].to_string()
        );
        res.push(s);

        // Calcualte the new state by moving the current number in forward direction
        num = ch.to_digit(10).unwrap() as i32;
        if num == 9 {
            num = 0;
        } else {
            num = num - 0 + 1;
        }
        let s = format!(
            "{}{}{}",
            src[0..i].to_string(),
            num,
            src[i + 1..].to_string()
        );
        res.push(s);
    }
    return res;
}

fn power_up(dead_states: Vec<String>, target: String) -> i32 {
    let mut q: VecDeque<String> = VecDeque::new();
    let mut visited: HashSet<String> = dead_states.iter().cloned().collect();
    let mut depth = -1;
    q.push_front("0000".to_string());
    while !q.is_empty() {
        depth += 1;
        let size = q.len();
        for _i in 0..size {
            let node: String = q.back().unwrap().to_string();
            q.pop_back();
            if node == target {
                return depth;
            }
            if visited.contains(&node) {
                continue;
            }
            visited.insert(node.clone());
            let neighbors = neighbors(node);
            for n in neighbors.into_iter() {
                q.push_front(n);
            }
        }
    }
    return -1;
}

fn main() {
    // Driver code

    let dead_states: Vec<String> = vec![
        "0201".to_string(),
        "0101".to_string(),
        "0102".to_string(),
        "1212".to_string(),
        "2002".to_string(),
    ];
    let target = String::from("0202");
    println!(
        "{}{:}{}",
        "The system will power up in ",
        power_up(dead_states, target),
        " dial turns\n"
    );
}

// Time complexity O(n^2*A^2+D)
// Space complexity = O(A^N + D)
