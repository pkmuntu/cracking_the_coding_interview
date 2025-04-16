use std::collections::HashMap;
fn find_duplicate_tweets( tweets_info: Vec<String>)-> Vec<Vec<String>> {
    // create a hashmap to store the duplicate people names for a hashtag
    let mut map: HashMap<String, Vec<String>> = HashMap::new();

    // iterate over each tweet information
    for  j in 0..tweets_info.len() {
        let tweet = tweets_info[j].to_string();

        // obtain the days, people names, and hashtags 
        // separately by splitting based on the space
         // utility function to split a string based on a delim
        let values: Vec<String> = tweet.split(" ").map(|s| s.to_string()).collect();
        for i in 1..values.len() {
          //println!("a");
            // split the person name and the hashtag appropriately to get the hashtag
            // utility function to split a string based on a delim
            let mut name_cont: Vec<String> = values[i].split("(").map(|s| s.to_string()).collect();
             name_cont[1].pop();
            // check if the hashtag already exists in the map
            // if it exists, then return its value and add the hashmap path to it,
            // otherwise create a new list and add the hashmap path to it
            let s = format!("{}{}{}",values[0], "/" , name_cont[0]);
            let mut list: Vec<String> = Vec::new();
            if map.contains_key(&name_cont[1]){
              list = map.get(&name_cont[1]).unwrap().to_vec();
              list.push(s);
              *map.get_mut(&name_cont[1]).unwrap() = list;
            }
            else{
              list.push(s);
              map.entry(name_cont[1].to_string()).or_insert(list);
            }
        }
    }
    
    // check if each hashtag has a list that consists of at least 
    // two hashtag paths, and add such lists to the output and return it
    let mut output: Vec<Vec<String>> = Vec::new();
    for itr in map.values() {
        if itr.len() > 1 {
            output.push(itr.to_vec());
        }
    }
    return output;
}

fn main() {
    // Driver code
    let tweets_info: Vec<String> = vec!["Monday Joe(t20) Jack(chevrolet) Charlie(ev)", "Tuesday Alice(cake) Bob(chevrolet)", "Wednesday Joe(boeing) Jack(ev) Alice(t20)"].into_iter().map(String::from).collect();

    let output = find_duplicate_tweets(tweets_info);
    println!("{:?}",output);
}

// Time complexity = O(n*x)
// Space complexity = o(n*x)
