use std::cmp;

fn partition_labels(files: &String) -> i32 {
    // Compute the last occurece of each latter
    let mut last: Vec<i32> = vec![0; 25];
    for (i, f) in files.chars().enumerate() {
        let v = f as u32 - 'a' as u32;
        last[v as usize] = i as i32;
    }

    let mut end = 0;
    let mut count = 0;
    let mut _start = 0;

    // Traverse the string
    for i in 0..files.len() {
        // Compute the highest last occurence position
        end = cmp::max(
            end,
            last[(files.chars().nth(i).unwrap() as u32 - 'a' as u32) as usize],
        );
        // Clause for when we reach the highest last occurence postion
        if i == end as usize {
            count += 1;
            _start = i + 1;
        }
    }
    return count;
}

fn main() {
    // Driver code

    let files: String = "abacdc".to_string();

    println!(
        "{}{}{}{:}{}",
        "The files \"",
        files,
        "\" will be divided into ",
        partition_labels(&files),
        " worker nodes!\n"
    );
}

// Time complexity = O(n)
// Space complexity = O(1)
