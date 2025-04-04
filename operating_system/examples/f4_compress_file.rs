use std::collections::HashMap;
use std::collections::HashSet;

fn dfs(word: String, word_set: HashSet<String>, mut cache: &mut HashMap<String, bool>) -> bool {
    // If result for current word already calculated then return from cache
    if cache.contains_key(&word) {
        return *cache.get(&word).unwrap();
    }

    // Traverse over the word to generate all combination
    for i in 1..word.len() {
        // Divide the word into prefix and suffix
        let prefix: String = word[0..i].to_string();
        let suffix: String = word[i..].to_string();

        if word_set.contains(&prefix) {
            if word_set.contains(&suffix) || dfs(suffix, word_set.clone(), &mut cache) {
                if cache.contains_key(&word) {
                    *cache.get_mut(&word).unwrap() = true;
                } else {
                    cache.entry(word).or_insert(true);
                }
                return true;
            }
        }
    }
    if cache.contains_key(&word) {
        *cache.get_mut(&word).unwrap() = false;
    } else {
        cache.entry(word).or_insert(false);
    }
    return false;
}

fn identify_concatenations(words: Vec<String>) -> Vec<String> {
    let mut res: Vec<String> = Vec::new();
    // Set for O(1) lookups
    let word_set: HashSet<String> = words.iter().cloned().collect();
    let mut cache: HashMap<String, bool> = HashMap::new();

    // Process for each word
    for word in words.into_iter() {
        if dfs(word.clone(), word_set.clone(), &mut cache) {
            res.push(word);
        }
    }
    return res;
}

fn main() {
    // Driver code
    let file_words: Vec<String> = vec!["n", "cat", "cats", "dog", "catsndog"]
        .into_iter()
        .map(String::from)
        .collect();
    let result = identify_concatenations(file_words);
    println!("{}{:?}", "The following words will be compressed: ", result);
}

// Time complexity = O(n * m^2)
// Space complexity = O(n * m^2)
