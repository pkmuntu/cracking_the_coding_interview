fn reconstruct_queue(process: &mut Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut temp: Vec<(i32, i32)> = Vec::new();
    let mut temp2: Vec<(i32, i32)> = vec![(0, 0); process.len()];

    for k in 0..process.len() {
        temp.push((process[k][0], process[k][1]));
    }
    // First sort priorities by priority and then by the k value.
    // priority in descending order and k value in ascending order.
    temp.sort();
    temp.reverse();
    for i in 0..temp.len() - 1 {
        if temp[i].0 == temp[i + 1].0 {
            if temp[i].1 > temp[i + 1].1 {
                let t = temp[i];
                temp[i] = temp[i + 1];
                temp[i + 1] = t;
            }
        }
    }
    for i in 0..temp.len() {
        temp2.insert(temp[i].1 as usize, temp[i]);
    }
    // Place the result back in original 2d array
    for l in 0..process.len() {
        process[l][0] = temp2[l].0;
        process[l][1] = temp2[l].1;
    }
    return process.to_vec();
}

fn main() {
    let mut p: Vec<Vec<i32>> = vec![
        vec![7, 0],
        vec![4, 4],
        vec![7, 1],
        vec![5, 0],
        vec![6, 1],
        vec![5, 2],
    ];
    let sol = reconstruct_queue(&mut p);
    println!("{:?}", sol);
}

// Time complexity = O(n^2)
// Space complexity = O(n)
