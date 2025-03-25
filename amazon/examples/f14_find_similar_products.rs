fn intersection(products_ids1: Vec<i32>, products_ids2: Vec<i32>) -> Vec<i32> {
    let mut counter: Vec<i32> = vec![0; 1001];
    // for (int i = 0; i < counter.Length; i++) {
    //   counter[i] = 0;
    // }

    for i in 0..products_ids1.len() {
        if counter[products_ids1[i] as usize] == 0 {
            counter[products_ids1[i] as usize] = 1;
        }
    }

    for i in 0..products_ids2.len() {
        if counter[products_ids2[i] as usize] == 1 {
            counter[products_ids2[i] as usize] = 2;
        }
    }

    let mut similar_purchases: Vec<i32> = Vec::new();

    for i in 0..counter.len() {
        if counter[i] > 1 {
            similar_purchases.push(i as i32);
        }
    }

    return similar_purchases.to_vec();
}

fn main() {
    // Driver code
    let products_ids1: Vec<i32> = vec![10, 100, 200, 300, 505, 606, 20, 100, 1, 5];
    let products_ids2: Vec<i32> = vec![200, 100, 300, 600, 100, 10, 1, 1, 505, 505, 606, 606];

    let similar_purchases = intersection(products_ids1, products_ids2);
    println!("{:?}", similar_purchases);
}

// Time complexity = O(n + m)
// Space complexity = o(1)
