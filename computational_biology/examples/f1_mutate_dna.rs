use std::collections::HashMap;

fn mutate_dna(s1: String, s2: String) -> bool {
    if s1.len() != s2.len() {
        return false;
    }
    if s1 == s2 {
        return true;
    }

    //  form the graph, we can represent it as a map descrbing the edges
    let mut edges: HashMap<char, char> = HashMap::new();
    for i in 0..s1.len() {
        let temp: char;
        // This clause corresponds to discovering more than one out-degree,
        // which we concluded is not possible
        if edges.contains_key(&s1.chars().nth(i).unwrap()) {
            temp = *edges.get(&s1.chars().nth(i).unwrap()).unwrap();
        } else {
            temp = s2.chars().nth(i).unwrap();
        }

        if temp != s2.chars().nth(i).unwrap() {
            return false;
        }
        // This corresponds to discovering a new edge
        edges
            .entry(s1.chars().nth(i).unwrap())
            .or_insert(s2.chars().nth(i).unwrap());
    }

    return edges.len() < 26;
}

fn main() {
    // Driver code

    let s1 = String::from("aabcc");
    let s2 = String::from("ccdee");

    if mutate_dna(s1, s2) {
        println!("Mutation Possible");
    } else {
        println!("Mutation not Possible");
    }
}

// Time complexity = O(n)
// Space complexity = O(n)
