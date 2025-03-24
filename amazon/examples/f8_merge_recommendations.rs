use std::collections::HashMap;
use std::collections::HashSet;
fn accounts_merge(accounts: Vec<Vec<String>>) -> Vec<Vec<String>> {
    let mut email_to_name: HashMap<String, String> = HashMap::new();
    let mut graph: HashMap<String, HashSet<String>> = HashMap::new();
    for acc in accounts.into_iter() {
        let name = &acc[0];
        for i in 1..acc.len() {
            let email = &acc[i];

            if graph.contains_key(&acc[1]) {
                graph.get_mut(&acc[1]).unwrap().insert(email.to_string());
            } else {
                let mut l: HashSet<String> = HashSet::new();
                l.insert(email.to_string());
                graph.insert(acc[1].clone(), l);
            }

            if graph.contains_key(email) {
                graph.get_mut(email).unwrap().insert(acc[1].clone());
            } else {
                let mut l: HashSet<String> = HashSet::new();
                l.insert(acc[1].clone());
                graph.insert(email.to_string(), l);
            }
            email_to_name
                .entry(email.to_string())
                .or_insert(name.to_string());
        }
    }

    let mut seen: HashSet<String> = HashSet::new();
    let mut ans: Vec<Vec<String>> = Vec::new();
    for (gkey, _gval) in graph.iter() {
        let email = gkey;
        if !seen.contains(email) {
            seen.insert(email.to_string());
            let mut stack: Vec<String> = Vec::new();
            stack.push(email.to_string());
            let mut component: Vec<String> = Vec::new();
            while !stack.is_empty() {
                let node: String = stack.last().unwrap().to_string();
                stack.pop();
                component.push(node.clone());
                for nei in graph.get(&node).unwrap().iter() {
                    if !seen.contains(nei) {
                        seen.insert(nei.to_string());
                        stack.push(nei.to_string());
                    }
                }
            }

            component.push(email_to_name[email].to_string());
            component.sort();
            ans.push(component);
        }
    }
    return ans;
}

fn main() {
    // Driver Code
    let accounts: Vec<Vec<String>> = vec![
        vec![
            "Sarah",
            "sarah22@email.com",
            "sarah@gmail.com",
            "sarahhoward@email.com",
        ]
        .into_iter()
        .map(String::from)
        .collect(),
        vec![
            "Alice",
            "alicexoxo@email.com",
            "alicia@email.com",
            "alicelee@gmail.com",
        ]
        .into_iter()
        .map(String::from)
        .collect(),
        vec!["Sarah", "sarah@gmail.com", "sarah10101@gmail.com"]
            .into_iter()
            .map(String::from)
            .collect(),
        vec!["Sarah", "sarah10101@gmail.com", "misshoward@gmail.com"]
            .into_iter()
            .map(String::from)
            .collect(),
    ];
    let result = accounts_merge(accounts);
    println!("{:?}", result);
}

// Time complexity = O(m+n)
// Space complexity = O(m+n)
