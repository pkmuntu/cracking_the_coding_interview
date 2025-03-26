fn inorder_bst(array: Vec<String>, n: i32) -> bool {
    // Return true if array has one or no element
    if n == 0 || n == 1 {
        return true;
    }

    // Found the unsorted pair
    for i in 1..n {
        if array[i as usize - 1] > array[i as usize] {
            return false;
        }
    }
    return true;
}

fn main() {
    let array: Vec<String> = vec![
        "Caryl", "Elia", "Elvira", "Jeanette", "Lala", "Latasha", "Lyn",
    ]
    .into_iter()
    .map(String::from)
    .collect();
    let n = array.len() as i32;
    if inorder_bst(array, n) {
        println!("Valid BST");
    } else {
        println!("Not valid BST");
    }
}

// Time complexity = O(m * n)
// Space complexity = O(1)
