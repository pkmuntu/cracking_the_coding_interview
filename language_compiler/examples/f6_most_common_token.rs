use std::collections::HashMap;
use std::collections::HashSet;

fn most_common_token(code: String, keywords: Vec<String>) -> String {
    // Replacing the syntax with space
    // Convert all the characters other than Alphanumeric into spaces
    // and insert them into normalized_code variable
    let normalized_code: String = code
        .chars()
        .map(|c| {
            if c.is_alphabetic() {
                c.to_ascii_lowercase()
            } else {
                ' '
            }
        })
        .collect();

    // Split based the spaces
    let tokens = normalized_code.split_whitespace();
    let mut count: HashMap<String, i32> = HashMap::new();
    let banned_words: HashSet<String> = keywords.into_iter().collect();
    // count occurence of each token, excluding the keywords
    for token in tokens.into_iter() {
        if !banned_words.contains(&token.to_string()) {
            if !count.contains_key(&token.to_string()) {
                count.entry(token.to_string()).or_insert(0);
            }
            *count.get_mut(&token.to_string()).unwrap() += 1;
        }
    }
    let key_with_max_value = count.iter().max_by_key(|entry| entry.1).unwrap();
    return key_with_max_value.0.to_string();
}

fn main() {
    // Driver code
    let code = String::from(
        "int main() {
    int value = getValue();  
    int sum = value + getRandom();
    int subs = value - getRandom();
    return 0;
}",
    );
    let keywords: Vec<String> = vec!["int", "main", "return"]
        .into_iter()
        .map(String::from)
        .collect();
    println!("{}", most_common_token(code, keywords));
}

// Time complexity = O(n + m)
// Sapce complexity = O(n + m)
