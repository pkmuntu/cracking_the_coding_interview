use rand::Rng;
use std::collections::HashMap;

#[derive(Default)]
pub struct UpsellProducts {
    product_dict: HashMap<i32, i32>,
    product_list: Vec<i32>,
}

impl UpsellProducts {
    fn new() -> Self {
        Default::default()
    }

    // insert a product to the dataset. Returns true if the dataset did not already contains
    // the specified product
    fn insert_product(&mut self, prod: i32) -> bool {
        if self.product_dict.contains_key(&prod) {
            return false;
        }

        self.product_dict
            .entry(prod)
            .or_insert(self.product_list.len() as i32);
        self.product_list.push(prod);
        return true;
    }

    // Remove a product from the dataset. Returns true if the dataset contained the
    // specified product
    fn remove_product(&mut self, prod: i32) -> bool {
        if !self.product_dict.contains_key(&prod) {
            return false;
        }

        let last = self.product_list[self.product_list.len() - 1];
        let index = self.product_dict.get(&prod).unwrap().clone();
        self.product_list[index as usize] = last;
        self.product_dict.entry(last).or_insert(index);
        self.product_list.pop();
        self.product_dict.remove(&prod);
        return true;
    }

    // Get a random product from the dataset
    fn get_random_product(&mut self) -> i32 {
        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0..self.product_list.len());
        return self.product_list[index];
    }
}

fn main() {
    // Driver Code
    let mut dataset = UpsellProducts::new();
    dataset.insert_product(1212);
    dataset.insert_product(190);
    dataset.insert_product(655);
    dataset.insert_product(327);
    println!("{:?}", dataset.get_random_product());
    dataset.remove_product(190);
    dataset.remove_product(1212);
    println!("{:?}", dataset.get_random_product());
}

// method                   Time complexity          Space complexity
// get_random_product          O(1)                       O(1)
// insert_product              O(1) amortized             O(n)
// remove_product              O(1) amortized             O(n)
