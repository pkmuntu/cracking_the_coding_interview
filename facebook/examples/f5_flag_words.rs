fn repeated_letters(s: &String, ind: i32) -> i32 {
    let mut temp = ind;
    while temp < s.len() as i32
        && s.chars().nth(temp as usize).unwrap() == s.chars().nth(ind as usize).unwrap()
    {
        temp = temp + 1;
    }
    return temp - ind;
}

fn flag_words(s: &String, w: &String) -> bool {
    if s.is_empty() || w.is_empty() {
        return false;
    }

    let mut i: i32 = 0;
    let mut j: i32 = 0;

    while i < s.len() as i32 && j < w.len() as i32 {
        if s.chars().nth(i as usize).unwrap() == w.chars().nth(j as usize).unwrap() {
            let len1 = repeated_letters(s, i);
            let len2 = repeated_letters(w, j);
            if len1 < 3 && len1 != len2 || len1 >= 3 && len1 < len2 {
                return false;
            }
            i += len1;
            j += len2;
        } else {
            return false;
        }
    }
    return i == s.len() as i32 && j == w.len() as i32;
}

fn main() {
    // Driver code

    let S = String::from("mooooronnn"); // modified word
    let W = String::from("moron"); // original word

    if flag_words(&S, &W) {
        println!("Word Flagged");
        println!(
            "{}{}{}{}{}{}",
            "The Word \"", S, "\"", " is a possible morph of \"", W, "\""
        );
    } else {
        println!("Word Safe");
    }
}

// Time complexity = O(max(sl, wl))
// Space complexity = O(1)
