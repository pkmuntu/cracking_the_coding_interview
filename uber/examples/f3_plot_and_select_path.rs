use std::collections::HashMap;
use std::collections::HashSet;

fn backtrack_evaluate(
    city: HashMap<String, HashMap<String, f64>>,
    curr_node: String,
    target_node: String,
    acc_sum: f64,
    mut visited: &mut HashSet<String>,
) -> f64 {
    visited.insert(curr_node.clone());
    let mut ret = -1.0;
    let neighbors: HashMap<String, f64> = city.get(&curr_node.clone()).unwrap().clone();
    if neighbors.contains_key(&target_node) {
        ret = acc_sum + neighbors.get(&target_node).unwrap();
    } else {
        for (next_node, val) in neighbors.iter() {
            if visited.contains(next_node) {
                continue;
            }
            ret = backtrack_evaluate(
                city.clone(),
                next_node.to_string(),
                target_node.clone(),
                acc_sum + val,
                &mut visited,
            );
            if ret != -1.0 {
                break;
            }
        }
    }

    //unmark the visited, for the next backtracking
    visited.remove(&curr_node);
    return ret;
}

fn get_total_cost(
    gmap: Vec<Vec<String>>,
    path_costs: Vec<f64>,
    drivers: Vec<String>,
    user: String,
    results: &mut Vec<f64>,
) {
    let mut city: HashMap<String, HashMap<String, f64>> = HashMap::new();
    // Step 1). build the city from the gmap
    for i in 0..gmap.len() {
        let check_points: Vec<String> = gmap[i].to_vec();
        let source_node: String = check_points[0].to_string();
        let dest_node: String = check_points[1].to_string();

        let path_cost: f64 = path_costs[i];

        let c: HashMap<String, f64> = HashMap::new();
        if !city.contains_key(&source_node) {
            city.insert(source_node.clone(), c.clone());
        }

        if !city.contains_key(&dest_node) {
            city.entry(dest_node.clone()).or_insert(c.clone());
        }

        if !city
            .get_mut(&source_node)
            .unwrap()
            .contains_key(&dest_node.clone())
        {
            *city
                .get_mut(&source_node)
                .unwrap()
                .entry(dest_node.clone())
                .or_insert(path_cost);
        } else {
            *city
                .get_mut(&source_node)
                .unwrap()
                .get_mut(&dest_node.clone())
                .unwrap() = path_cost;
        }

        if !city
            .get_mut(&dest_node)
            .unwrap()
            .contains_key(&source_node.clone())
        {
            *city
                .get_mut(&dest_node)
                .unwrap()
                .entry(source_node.clone())
                .or_insert(path_cost);
        } else {
            *city
                .get_mut(&dest_node)
                .unwrap()
                .get_mut(&source_node.clone())
                .unwrap() = path_cost;
        }
    }

    // Step2). Evaluate each driver via backtrackig (DFS)
    // by verifying if there exists a path form driver to user
    for i in 0..drivers.len() {
        let driver: String = drivers[i].to_string();
        if !city.contains_key(&driver) || !city.contains_key(&user) {
            results[i] = -1.0;
        } else {
            let mut visited: HashSet<String> = HashSet::new();
            results[i] = backtrack_evaluate(city.clone(), driver, user.clone(), 0.0, &mut visited);
        }
    }
}

fn main() {
    // Driver code
    let gmap: Vec<Vec<String>> = vec![
        vec!["a".to_string(), "b".to_string()],
        vec!["b".to_string(), "c".to_string()],
        vec!["a".to_string(), "e".to_string()],
        vec!["d".to_string(), "e".to_string()],
    ];
    let path_costs: Vec<f64> = vec![12.0, 23.0, 26.0, 18.0];
    let drivers: Vec<String> = vec!["c", "d", "e", "f"]
        .into_iter()
        .map(String::from)
        .collect();
    let user = "a".to_string();
    let mut all_path_costs: Vec<f64> = vec![0.0; 4];
    get_total_cost(gmap, path_costs, drivers, user, &mut all_path_costs);

    println!("{}{:?}", "Total cost of all paths ", all_path_costs);
}

// Time complexity = O(m * n)
// Space compleixty = O(m + n)
