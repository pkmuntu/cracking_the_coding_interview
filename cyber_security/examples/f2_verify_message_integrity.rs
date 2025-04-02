use std::collections::HashMap;

fn verify_message_integrity(messages: &Vec<&str>, key: &str) -> bool {
    if messages.len() == 1 {
        return true;
    }

    let mut key_map: HashMap<char, usize> = HashMap::new();
    for (i, elem) in key.chars().enumerate() {
        key_map.entry(elem).or_insert(i);
    }

    for i in 0..messages.len() - 1 {
        for j in 0..messages[i].len() {
            // If we do not find a mismatch letter between messages[i] and messages[i+1],
            // we need to examine the case when messages are like ("educated", "educate").
            if j >= messages[i + 1].len() {
                return false;
            }
            let first = messages[i].chars().nth(j).unwrap();
            let second = messages[i + 1].chars().nth(j).unwrap();
            if first != second {
                if key_map[&first] > key_map[&second] {
                    return false;
                } else {
                    // if we find the first different character and they are sorted,
                    // then there's no need to check remaining letters
                    break;
                }
            }
        }
    }
    true
}

fn main() {
    let messages = vec!["alpha", "bravo", "charlie", "delta"];
    let key = "abcdlpite";
    println!("{}", verify_message_integrity(&messages, key));

    let messages = vec!["apple", "app"];
    let key = "apple";
    println!("{}", verify_message_integrity(&messages, key));
}

// Time complexity = O(m) m: toatl number of letters in the messages
// Space complexity = O(1)
