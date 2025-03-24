use std::collections::HashMap;
fn suggest_two_products(item_prices: Vec<i32>, amount: i32) -> Vec<i32> {
    let mut buff_dict: HashMap<i32, i32> = HashMap::new();
    for i in 0..item_prices.len() {
        let price = item_prices[i];
        let remaining = amount - item_prices[i];
        if !buff_dict.contains_key(&remaining) {
            buff_dict.entry(price).or_insert(i as i32);
        } else {
            return vec![*buff_dict.get(&remaining).unwrap(), i as i32];
        }
    }
    return vec![];
}

fn main() {
    let item_prices: Vec<i32> = vec![2, 30, 56, 34, 55, 10, 11, 20, 15, 60, 45, 39, 51];
    let amount = 61;
    let f = suggest_two_products(item_prices, amount);
    println!("{:?}", f);
}

// Time complexity = O(n)
// Space complexity = O(n0
