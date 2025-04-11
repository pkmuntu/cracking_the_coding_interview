use std::collections::HashMap;
use std::collections::HashSet;
fn dfs(
    w: String,
    end_word: String,
    graph: HashMap<String, HashSet<String>>,
    path: &mut Vec<String>,
    ans: &mut Vec<Vec<String>>,
) {
    if w == end_word {
        ans.push(path.to_vec());
    } else {
        for itr in graph.get(&w).unwrap().iter() {
            path.push(itr.to_string());
            dfs(itr.to_string(), end_word.clone(), graph.clone(), path, ans);
            path.pop();
        }
    }
}

fn possible_results(
    begin_word: String,
    end_word: String,
    word_list: Vec<String>,
    mut res: &mut Vec<Vec<String>>,
) {
    if begin_word == end_word {
        res.push(vec![begin_word]);
        return;
    }
    let mut graph: HashMap<String, HashSet<String>> = HashMap::new();
    let helper = String::from("qwertyuiopasdfghjklzxcvbnm");
    let mut dic_set: HashSet<String> = word_list.into_iter().collect();
    let mut layer: HashSet<String> = HashSet::new();
    layer.insert(begin_word.clone());
    dic_set.remove(&begin_word);

    let mut stop = false;
    while layer.len() > 0 && !stop {
        let mut new_layer: HashSet<String> = HashSet::new();
        for itr in layer.iter() {
            let w = itr.clone();

            for i in 0..w.len() {
                for c in helper.chars() {
                    let mut nw = w[0..i].to_string();
                    nw.push(c);
                    nw.push_str(&w[i + 1..]);
                    if dic_set.contains(&nw) {
                        new_layer.insert(nw.clone());
                        if graph.contains_key(&w) {
                            graph.get_mut(&w).unwrap().insert(nw.clone());
                        } else {
                            let mut l: HashSet<String> = HashSet::new();
                            l.insert(nw.clone());
                            graph.insert(w.clone(), l);
                        }
                    }
                    // if we find end_word in this layer, there is no need to do next layer
                    if nw == end_word {
                        stop = true;
                    }
                }
            }
        }
        // removing unecessary nodes

        dic_set = &dic_set - &new_layer;
        layer = new_layer;
    }

    let mut path: Vec<String> = vec![begin_word.clone()];
    dfs(begin_word, end_word, graph, &mut path, &mut res);
}

fn main() {
    // Driver Code
    let initial_word = String::from("hit");
    let final_word = String::from("cog");
    let word_group: Vec<String> = vec![
        "hot".to_string(),
        "dot".to_string(),
        "dog".to_string(),
        "lot".to_string(),
        "log".to_string(),
        "cog".to_string(),
        "lit".to_string(),
    ];
    let mut res: Vec<Vec<String>> = Vec::new();
    possible_results(initial_word, final_word, word_group, &mut res);
    println!("All minimum sequences are:");
    println!("{:?}", res);
}

// Time complexity = O((m*26)*n)
// Space complexity = O(m^2*n)
