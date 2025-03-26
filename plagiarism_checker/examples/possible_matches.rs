use std::collections::HashMap;

fn possible_matches(s: String, words: Vec<String>) -> i32 {
    let mut words = {
        let mut waiting_list = HashMap::new();
        for mut word in words {
            let head = word.remove(0);
            waiting_list.entry(head).or_insert(vec![]).push(word);
        }
        waiting_list
    };

    let mut ans = 0;
    for c in s.chars() {
        let advance = match words.remove(&c) {
            Some(a) => a,
            None => continue,
        };

        for mut word in advance {
            if word.len() == 0 {
                ans += 1;
                continue;
            }
            let head = word.remove(0);
            words.entry(head).or_insert(vec![]).push(word);
        }
    }
    ans
}

fn main() {
    // Driver code

    let plagiarised = String::from("abcde");
    let students: Vec<String> = vec![
        "a".to_string(),
        "bb".to_string(),
        "acd".to_string(),
        "ace".to_string(),
    ];

    println!(
        "{}{:}",
        "The content was copied from ",
        possible_matches(plagiarised, students)
    );
}
