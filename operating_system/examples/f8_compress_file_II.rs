fn compress(chars: &mut Vec<char>) -> Vec<char> {
    let mut i = 0;
    while i < chars.len() {
        let mut count = 1;
        let ch = chars[i];
        i += 1;
        let mut c = i;

        // count the number of times a character repeats
        while i < chars.len() && chars[i] == ch {
            chars.remove(i);
            count += 1;
        }

        //insert the count
        if count > 1 {
            let s = count.to_string();
            for sc in s.chars() {
                chars.insert(c, sc);
                c += 1;
            }
        }
    }
    return chars.to_vec();
}

fn main() {
    let mut chars: Vec<char> = vec!['a', 'a', 'b', 'b', 'b', 'c', 'a', 'a', 'a', 'a'];
    println!("{:?}", compress(&mut chars));
}

// Time complexity = O(n^2)
// Space complexity = O(1)
