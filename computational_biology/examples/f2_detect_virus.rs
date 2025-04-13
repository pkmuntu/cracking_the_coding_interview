use std::collections::HashMap;

fn detect_virus(s: String, k: i32) -> String {
    if s.len() as i32 * k == 0 {
        return "".to_string();
    }

    let mut left = 0;
    let mut right = 0;

    let mut start: i32 = 0;
    let mut end: i32 = 0;

    let mut character_map: HashMap<char, i32> = HashMap::new();

    while right < s.len() as i32 {
        if !character_map.contains_key(&s.chars().nth(right as usize).unwrap()) {
            character_map
                .entry(s.chars().nth(right as usize).unwrap())
                .or_insert(right);
        } else {
            *character_map
                .get_mut(&s.chars().nth(right as usize).unwrap())
                .unwrap() = right;
        }
        right += 1;

        //This clause checks if window contains more than k characters
        if character_map.len() == (k + 1) as usize {
            let min_idx = character_map.values_mut().min().unwrap();
            let min = *min_idx;
            left = *min_idx + 1;
            character_map.remove(&s.chars().nth(min as usize).unwrap());
            // move left pointer of the window
        }
        if (end - start) < (right - left) {
            start = left;
            end = right;
        }
    }
    return s[start as usize..end as usize].to_string();
}

fn main() {
    // Driver code
    let infected_dna = String::from("ababffzzeee");
    let k = 3; // Supplied from a hidden program

    println!("{}", detect_virus(infected_dna, k));
}

// Time complexity = O(Nk)
// Space complexity = O(k)
