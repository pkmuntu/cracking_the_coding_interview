use std::collections::HashMap;
use std::collections::VecDeque;

fn minimum_moves(word_list: Vec<String>, begin_word: String, end_word: String) -> i32 {
    // Since all words are of same length.
    let l = begin_word.len();
    let target = word_list.iter().position(|s| s == &end_word);
    if target.is_none() {
        return 0;
    }
    let mut q = VecDeque::new();

    // Dictionary to hold combination of words that can be formed,
    // from any given word. By changing one letter at a time.
    let mut states_list: HashMap<String, Vec<String>> = HashMap::new();

    for word in word_list.into_iter() {
        for i in 0..l {
            // Key is the generic word
            // Value is a list of words which have the same intermediate generic word.
            let new_word = format!(
                "{}{}{}",
                word[0..i].to_string(),
                "*",
                word[i + 1..].to_string()
            );
            let mut transformations: Vec<String> = Vec::new();
            if states_list.contains_key(&new_word) {
                transformations = states_list.get(&new_word).unwrap().to_vec();
            }
            transformations.push(word.clone());
            if !states_list.contains_key(&new_word) {
                states_list.entry(new_word).or_insert(transformations);
            } else {
                *states_list.get_mut(&new_word).unwrap() = transformations;
            }
        }
    }
    // queue for BFS

    q.push_front((begin_word.clone(), 0));

    //Visited to make sure we don't repeat processing same word.
    let mut visited: HashMap<String, bool> = HashMap::new();

    visited.entry(begin_word).or_insert(true);
    while !q.is_empty() {
        let curr_word = q.pop_back().unwrap();
        let word = curr_word.0;
        let level = curr_word.1;
        for i in 0..l {
            // Intermediate words for current word
            let new_word = format!(
                "{}{}{}",
                word[0..i].to_string(),
                "*",
                word[i + 1..].to_string()
            );
            // Next states are all the words which share the same intermediate state.
            let mut temp: Vec<String> = Vec::new();
            if states_list.contains_key(&new_word) {
                temp = states_list.get(&new_word).unwrap().to_vec();
            }
            for adjacent_word in temp.into_iter() {
                // If at any point if we find what we are looking for
                // i.e. the end word - we can return with the answer.
                if adjacent_word == end_word {
                    return level + 1;
                }
                // Otherwise, add it to the BFS queue. Also mark it visited
                if !visited.contains_key(&adjacent_word) {
                    visited.entry(adjacent_word.clone()).or_insert(true);
                    q.push_front((adjacent_word, level + 1));
                }
            }
        }
    }

    return 0;
}

fn main() {
    // Driver code

    let initial_word = String::from("hit");
    let final_word = String::from("cog");
    let word_group: Vec<String> = vec![
        "hot".to_string(),
        "dot".to_string(),
        "dog".to_string(),
        "lot".to_string(),
        "log".to_string(),
        "cog".to_string(),
    ];

    println!(
        "{}{:}",
        "The shortest sequece is of length: ",
        minimum_moves(word_group, initial_word, final_word)
    );
}

// Time complexity = O(m^2 * n)
// Space complexity = O(m^2 * n)
