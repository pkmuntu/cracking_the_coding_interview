use std::collections::HashMap;
#[derive(Default, Debug)]
pub struct TimeMap {
    pub prices: HashMap<String, Vec<String>>,
    pub timestamps: HashMap<String, Vec<i32>>,
}

impl TimeMap {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn set(&mut self, item: String, price: String, timestamp: i32) {
        // Check if the given item already exists in the data structure
        if self.prices.contains_key(&item) {
            // Check if the given timestamp is in increasing order
            if timestamp
                < self.timestamps.get(&item).unwrap()[self.timestamps.get(&item).unwrap().len() - 1]
            {
                println!(
                    "{}{:}{}",
                    "Timestamp is expected to have values ranging from ",
                    self.timestamps.get(&item).unwrap()
                        [self.timestamps.get(&item).unwrap().len() - 1],
                    " to 10000000 only."
                );
            // Check if the given price of an item already exists in the data structure
            } else if price
                != self.prices.get(&item).unwrap()[self.prices.get(&item).unwrap().len() - 1]
            {
                // Store prices for the given item in the data structure
                self.prices.get_mut(&item).unwrap().push(price);
                // Store timestamp for the given item in the data structure
                self.timestamps.get_mut(&item).unwrap().push(timestamp);
            }
        } else {
            // Store prices and item for the given item in the data structure
            self.prices.entry(item.to_string()).or_insert(vec![price]);
            // Store timestamp for the given item in the data structure
            self.timestamps
                .entry(item.to_string())
                .or_insert(vec![timestamp]);
        }
    }

    pub fn get(&mut self, item: String, timestamp: i32) -> String {
        // Check if the given item is present in the data structure
        if !self.prices.contains_key(&item) {
            return "".to_string(); // return empty string if item does not exist
        } else {
            let index = self
                .timestamps
                .get_mut(&item)
                .unwrap()
                .binary_search(&timestamp);

            if index.is_ok() {
                return self.prices.get(&item).unwrap()[index.unwrap()].to_string();
            } else {
                if index > Err(0) {
                    let c = self.prices.get(&item).unwrap().len();
                    return self.prices.get(&item).unwrap()[c - 1].to_string();
                }
            }
            return "".to_string();
        }
    }
}

fn main() {
    // driver code
    //Example #1
    println!("Example #1");

    let mut map: TimeMap = TimeMap::new();
    // set price
    println!("SAVE: Item = Sun Glasses; Price = $20; TimeStamp = 3");
    // set
    map.set("Sun Glasses".to_string(), "$20".to_string(), 3);
    println!("{:?}", map);
    // get
    println!("Get: Item = Sun Glasses; TimeStamp = 3");
    let mut output = map.get("Sun Glasses".to_string(), 3);
    println!("{}{:?}", "GOT: The price of Sun Glasses is: ", output);

    // get
    println!("Get: Item = Sun Glasses; TimeStamp = 1");
    output = map.get("Sun Glasses".to_string(), 1);
    println!("{}{:?}", "GOT: The price of Sun Glasses is: ", output);

    println!("SAVE: Item = Sun Glasses; Price = $15, TimeStamp = 4");
    // set
    map.set("Sun Glasses".to_string(), "$15".to_string(), 4);

    // get
    println!("Get: Item = Sun Glasses; TimeStamp = 8");
    output = map.get("Sun Glasses".to_string(), 8);
    println!("{}{:?}", "GOT: The price of Sun Glasses is: ", output);

    // get
    println!("Get: Item = Sun Glasses; TimeStamp = 4");
    output = map.get("Sun Glasses".to_string(), 4);
    println!("{}{:?}", "GOT: The price of Sun Glasses is: ", output);

    // // Example #2
    println!("Example #2");

    let mut map1: TimeMap = TimeMap::new();
    println!("SAVE: Item = Watch; Price = $30; TimeStamp = 5");
    map1.set("Watch".to_string(), "$30".to_string(), 5);

    println!("Get: Item = Watch; TimeStamp = 1");
    output = map1.get("Watch".to_string(), 1);
    println!("{}{:?}", "GOT: The price of Watch is: ", output);

    println!("Get: Item = Watch; TimeStamp = 5");
    output = map1.get("Watch".to_string(), 5);
    println!("{}{:?}", "GOT: The price of Watch is: ", output);
    // set
    println!("SAVE: Item = Watch; Price = $25 ; TimeStamp = 2");
    map1.set("Watch".to_string(), "$25".to_string(), 2);

    println!("Get: Item = Watch; TimeStamp = 5");
    output = map1.get("Watch".to_string(), 5);
    println!("{}{:?}", "GOT: The price of Watch is: ", output);
    println!("Get: Item = Watch; TimeStamp = 3");
    output = map1.get("Watch".to_string(), 3);
    println!("{}{:?}", "GOT: The price of Watch is: ", output);
}

//  Method               Time complexity           Space complexity
//  Set                      O(1)                       O(n)
//  get                      O(logn)
