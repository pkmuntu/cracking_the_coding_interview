fn reverse_words(s: &mut String) -> String {
    // Remove leading and trailing spaces
    *s = s.trim().to_string();

    // Split the string by multiple spaces
    let mut word_list: Vec<&str> = s.split(" ").collect();
    // Reverse the word_list
    word_list.reverse();

    return word_list.join(" ");
}

fn main() {
    println!(
        "{}",
        reverse_words(&mut " *.html pages.tar.gz czvf tar ".to_string())
    );
}

// Time complexity = O(n)
// Space complexity = O(n)
