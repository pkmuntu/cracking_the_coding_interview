use std::collections::HashMap;

fn find_similarity(products: Vec<i32>, condidates: Vec<i32>) -> Vec<i32> {
    let prod_n = products.len();
    let cand_n = condidates.len();

    if prod_n < cand_n {
        return vec![0];
    }

    let mut cand_count: HashMap<i32, i32> = HashMap::new();
    let mut prod_count: HashMap<i32, i32> = HashMap::new();

    let mut output: Vec<i32> = Vec::new();

    for i in condidates {
        if cand_count.contains_key(&i) {
            *cand_count.get_mut(&i).unwrap() += 1;
        } else {
            cand_count.entry(i).or_insert(1);
        }
    }

    for i in 0..prod_n {
        let mut k = products[i];
        if prod_count.contains_key(&k) {
            *prod_count.get_mut(&k).unwrap() += 1;
        } else {
            prod_count.entry(k).or_insert(1);
        }

        if i >= cand_n {
            k = products[i - cand_n] as i32;
            if *prod_count.get(&k).unwrap() == 1 {
                prod_count.remove(&k);
            } else {
                *prod_count.get_mut(&k).unwrap() -= 1;
            }
        }
        if cand_count == prod_count {
            output.push((1 + i - cand_n) as i32);
        }
    }
    return output;
}

fn main() {
    let products: Vec<i32> = vec![3, 2, 1, 5, 2, 1, 2, 1, 3, 4];
    let candidates: Vec<i32> = vec![1, 2, 3];
    let result: Vec<i32> = find_similarity(products, candidates);
    println!("{:?}", result);
}

// Time complexity = O(n+m)
// Space complexity = O(m)
