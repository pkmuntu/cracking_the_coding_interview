use std::collections::HashSet;

fn two_products(item_prices: &mut Vec<i32>, i: i32, res: &mut Vec<Vec<i32>>) {
    let mut seen: HashSet<i32> = HashSet::new();
    let mut j: usize = i as usize + 1;
    while j < item_prices.len() {
        let complement = 200 - item_prices[i as usize] - item_prices[j];
        if seen.contains(&complement) {
            res.push([item_prices[i as usize], item_prices[j], complement].to_vec());
            if j + 1 < item_prices.len() && item_prices[j] == item_prices[j + 1] {
                j += 1;
            }
        }
        seen.insert(item_prices[j]);
        j += 1;
    }
}

fn suggest_three_products(item_prices: &mut Vec<i32>, res: &mut Vec<Vec<i32>>) {
    item_prices.sort();
    for i in 0..item_prices.len() {
        let price = item_prices[i];
        if price > 200 {
            break;
        }
        if i == 0 || item_prices[i - 1] != item_prices[i] {
            two_products(item_prices, i as i32, res);
        }
    }
}

// Driver code
fn main() {
    let mut item_prices: Vec<i32> = vec![100, 75, 150, 200, 50, 65, 40, 30, 15, 25, 60];
    let mut res: Vec<Vec<i32>> = Vec::new();
    suggest_three_products(&mut item_prices, &mut res);
    println!("{:?}", res);
    //return 0;
}

// Time complexity = O(n^2)
// Space complexity = O(n)
