use std::collections::HashMap;
use std::collections::HashSet;

fn helper(
    s: String,
    dictset: HashSet<String>,
    map: &mut HashMap<String, Vec<String>>,
    res: &mut Vec<String>,
) -> Vec<String> {
    if map.contains_key(&s) {
        *res = map.get(&s).unwrap().to_vec();
        return res.to_vec();
    }

    if s.len() == 0 {
        res.push("".to_string());
        return res.to_vec();
    }

    for word in dictset.clone().into_iter() {
        if s.starts_with(word.as_str()) {
            if word.len() == s.len() {
                res.push(word);
            } else {
                let mut sublist: Vec<String> = Vec::new();
                helper(
                    s[word.len()..].to_string(),
                    dictset.clone(),
                    map,
                    &mut sublist,
                );
                for sub in sublist.into_iter() {
                    let mut s = "".to_string();
                    s.push_str(&word.clone());
                    if sub.is_empty() {
                        s.push_str(&"".to_string());
                    } else {
                        s.push_str(&" ".to_string());
                    }
                    s.push_str(&sub);
                    res.push(s.clone());
                }
            }
        }
    }
    if map.contains_key(&s) {
        *map.get_mut(&s).unwrap() = res.to_vec();
    } else {
        map.entry(s).or_insert(res.to_vec());
    }
    return res.to_vec();
}

fn break_query(query: String, dict: Vec<String>, res: &mut Vec<String>) {
    let dictset: HashSet<String> = dict.into_iter().collect();
    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    *res = helper(query.clone(), dictset, &mut map, &mut res.to_vec()).to_vec();
}

fn main() {
    let mut query = String::from("vegancookbook");
    let dict: Vec<String> = vec![
        "an", "book", "car", "cat", "cook", "cookbook", "crash", "cream", "high", "highway", "i",
        "ice", "icecream", "low", "scream", "veg", "vegan", "way",
    ]
    .into_iter()
    .map(String::from)
    .collect();
    let mut res: Vec<String> = Vec::new();
    break_query(query, dict.clone(), &mut res);
    println!("{:?}", res);
    query = "highwaycarcrash".to_string();
    res.clear();
    break_query(query, dict, &mut res);
    println!("{:?}", res);
}

// Time complexity = O(n^2 + 2^m + w)
// Space complextiy = O((n * 2^n) + l)
