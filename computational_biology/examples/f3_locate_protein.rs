use std::cmp;

fn return_palindrome_length(s: String, left: i32, right: i32) -> i32 {
    let mut r = right;
    let mut l = left;

    while l >= 0
        && r < s.len() as i32
        && s.chars().nth(l as usize).unwrap() == s.chars().nth(r as usize).unwrap()
    {
        l -= 1;
        r += 1;
    }

    return r - l - 1;
}

fn locate_protein(s: &mut String) -> String {
    if s.len() < 1 {
        return "".to_string();
    }

    let mut start = 0;
    let mut end = 0;

    for i in 0..s.len() {
        let len1 = return_palindrome_length(s.to_string(), i as i32, i as i32);
        let len2 = return_palindrome_length(s.to_string(), i as i32, (i + 1) as i32);

        let len = cmp::max(len1, len2);

        if len > end - start {
            start = i as i32 - (len - 1) / 2;
            end = i as i32 + len / 2;
        }
    }
    end = end + 1;
    return s[start as usize..end as usize].to_string();
}

fn main() {
    // Driver code
    let mut sequence = String::from("aaccbababcbc");

    println!("{}", locate_protein(&mut sequence));
}

// Time complexity = O(n^2)
// Space complexity = O(1)
