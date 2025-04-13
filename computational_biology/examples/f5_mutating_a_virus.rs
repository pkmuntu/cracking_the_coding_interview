fn next_mutation(mut num: &mut Vec<i32>) -> Vec<i32> {
    // Get the index of the second last number and compare it with the
    // number next to it, on the right
    let mut index: i32 = num.len() as i32 - 2;

    // Find the index of the number num[index -1] which satisfies
    // the condition num[index] > num[index-1]. Here num[index]
    // and nun[index + 1] represent two successive numbers in the sequence.
    while index >= 0 && num[index as usize] >= num[index as usize + 1] {
        index -= 1;
    }

    // Find the the number num[j] which is just larger than num[index -1]
    // amont the numbers lying to the right of num[index -1]
    if index >= 0 {
        let mut j: i32 = num.len() as i32 - 1;
        while num[j as usize] <= num[index as usize] {
            j -= 1;
        }

        // Swap num[index - 1] and num[j]
        swap_numbers(&mut num, index, j);
    }

    // Reverse the numbers following num[index -1], so that they are in
    // ascending order.
    reverse_list(&mut num, index + 1);
    return num.to_vec();
}

fn reverse_list(mut num: &mut Vec<i32>, start: i32) {
    let mut i = start;
    let mut j = num.len() as i32 - 1;

    while i < j {
        swap_numbers(&mut num, i, j);
        i += 1;
        j -= 1;
    }
}

fn swap_numbers(num: &mut Vec<i32>, i: i32, j: i32) {
    let temp = num[i as usize];
    num[i as usize] = num[j as usize];
    num[j as usize] = temp;
}

fn main() {
    // Driver code
    let mut num: Vec<i32> = vec![4, 5, 2, 6, 7, 3, 1];
    println!("{}{:?}", "Input: ", num);
    next_mutation(&mut num);
    println!("{}{:?}", "Output: ", num);
}

// Time complexity = O(n)
// Space complexity = O(1)
