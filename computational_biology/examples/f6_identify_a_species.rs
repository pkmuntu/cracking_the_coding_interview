use std::collections::HashMap;
fn find_species_marker(nucleotide: String) -> String {
    // Check if a DNA sequence has no nucleotide
    if nucleotide.len() == 0 {
        return "DNA sequence must have atleast one nucleotide.".to_string();
    }

    let n = nucleotide.len();
    let mut st_curr: i32 = 0;
    let mut longest: i32 = 0;
    let mut curr_len: i32;
    let mut start: i32 = 0;
    let i;
    let mut window: HashMap<char, i32> = HashMap::new();

    // Traverse the DNA sequence to find the longest substring
    // without repeating characters.
    for i in 0..n {
        // If the current nucleotide is not present in the hash table,
        // then store it in the hash table with the value as the current index.
        if !window.contains_key(&nucleotide.chars().nth(i).unwrap()) {
            window
                .entry(nucleotide.chars().nth(i).unwrap())
                .or_insert(i as i32);
        } else {
            // If the current nucleotide is present in the hash table,
            // it means that this nucleotide can be repeated.
            // Check if the current nucleotide occurs before or after `st_curr`.
            if window.get(&nucleotide.chars().nth(i).unwrap()).unwrap() >= &st_curr {
                curr_len = i as i32 - st_curr;
                if longest < curr_len {
                    longest = curr_len;
                    start = st_curr;
                }

                // The next substring will start after the last
                // occurence of the current nucleotide.
                st_curr = window.get(&nucleotide.chars().nth(i).unwrap()).unwrap() + 1;
            }

            // Update the last occurence of
            // the nucleotide in the hash table
            *window.get_mut(&nucleotide.chars().nth(i).unwrap()).unwrap() = i as i32;
        }
    }

    // Update the longest substring's
    // Length and starting index.
    i = n as i32;
    if longest < i - st_curr {
        longest = i - st_curr;
        start = st_curr;
    }

    return nucleotide[start as usize..(start + longest) as usize].to_string();
}

fn main() {
    //Driver code
    let nucleotide = "abcdbea".to_string();
    let strr = find_species_marker(nucleotide);
    println!("{}{}", "Specie marker: ", strr);
    println!("{}{:}", "Length of specie marker: ", strr.len());
}

// Time complexity = O(n)
// Space complexity = O(n)
