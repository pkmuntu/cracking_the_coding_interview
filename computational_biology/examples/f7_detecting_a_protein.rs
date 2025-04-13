use std::collections::HashMap;

fn is_protein(s: String) -> bool {
    let mut map: HashMap<char, i32> = HashMap::new();

    for i in 0..s.len() {
        let occurence = 1;
        if !map.contains_key(&s.chars().nth(i).unwrap()) {
            map.entry(s.chars().nth(i).unwrap()).or_insert(occurence);
        } else {
            *map.get_mut(&s.chars().nth(i).unwrap()).unwrap() += occurence;
        }
    }

    let mut count = 0;

    for (nuc, _val) in map.iter() {
        if !map.get(nuc).unwrap() % 2 == 0 {
            count += 1;
        }
    }

    if count == 1 || count == 0 {
        return true;
    } else {
        return false;
    }
}

fn main() {
    let sequence = "baefeab".to_string();
    let isprotein: bool = is_protein(sequence.to_string());
    println!("{}{}", "Input: ", sequence.to_string());
    println!("{}{:}", "Output: ", isprotein);

    let sequence = "abc".to_string();
    let isprotein: bool = is_protein(sequence.to_string());
    println!("{}{}", "Input: ", sequence.to_string());
    println!("{}{:}", "Output: ", isprotein);
}

// Time complexity = O(n)
// Space complexity = O(1)
