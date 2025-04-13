fn find_kth_missing_gene(a: Vec<i32>, k: i32) -> i32 {
    let mut left: i32 = 0;
    let mut right: i32 = a.len() as i32 - 1;

    while left <= right {
        // we choose the pivot, which is the middle index of the array, a
        let pivot = (left + right) / 2;

        // If the number of missing genes before the current gene
        // is less than k, we will continue to search on the righ side of A
        if a[pivot as usize] - pivot - 1 < k {
            left = pivot + 1;
        } else {
            right = pivot - 1;
        }
    }
    return left + k;
}

fn main() {
    let a: Vec<i32> = vec![2, 3, 4, 7, 11];
    let k = 5;

    let missing_genes = find_kth_missing_gene(a, k);

    println!("{}{:}", "The kth missing gene is: ", missing_genes);
}

// Time complexity = O(log n)
// Space complexity = O1)
