fn num_plain_text(c: String) -> i32 {
    let mut memo = vec![0; c.len() + 1];

    for i in 0..memo.len() {
        memo[i] = -1;
    }

    return memoization(0, c, &mut memo);
}

fn memoization(index: i32, c: String, mut memo: &mut Vec<i32>) -> i32 {
    if memo[index as usize] != -1 {
        return memo[index as usize];
    }

    // If we reach the end of the ciphertext, we return 1.
    if index == c.len() as i32 {
        return 1;
    }

    // If the ciphertext starts with a zero, it can't be deciphered
    if c.chars().nth(index as usize).unwrap() == '0' {
        return 0;
    }

    if index == (c.len() - 1) as i32 {
        return 1;
    }

    // make a recursive call to the function with index + 1 for next substring
    let mut result = memoization(index + 1, c.to_string(), &mut memo);

    // make a recursive call to the function with index + 2 after checking for
    // valid two-digit decipher

    if c[index as usize..index as usize + 2]
        .to_string()
        .parse::<i32>()
        .unwrap()
        <= 26
    {
        result += memoization(index + 2, c.to_string(), &mut memo);
    }

    // save the result to be used later, in case of overlapping subproblems.
    memo[index as usize] = result;

    return result;
}

fn main() {
    // Driver code
    let c = "2317".to_string();
    let num = num_plain_text(c);
    println!("{}{:}", "Number of possible plain text: ", num);
}

// Time complexity = O(n)
// Space complexity = O(n)
