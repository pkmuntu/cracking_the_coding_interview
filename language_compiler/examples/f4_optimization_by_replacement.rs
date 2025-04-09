fn optimize_line(
    line: &mut String,
    indices: Vec<i32>,
    sources: Vec<String>,
    targets: Vec<String>,
) -> String {
    let mut sorted: Vec<Vec<i32>> = Vec::new();
    for i in 0..indices.len() {
        sorted.push([indices[i], i as i32].to_vec());
    }
    sorted.sort();
    sorted.reverse();
    for ind in sorted.into_iter() {
        let i = ind[0] as usize;
        let j = ind[1];
        let s = sources[j as usize].to_string();
        let t: String = targets[j as usize].to_string();
        let len = s.len();
        if &line[i..i + len] == s {
            let ll = format!("{}{}{}", &line[0..i], &t, &line[i + len..]);
            *line = ll;
        }
    }
    return line.clone();
}

fn main() {
    let mut line = String::from("foo(input, i);");
    let indices: Vec<i32> = vec![0, 11];
    let sources: Vec<String> = vec!["foo".to_string(), "i".to_string()];
    let targets: Vec<String> = vec!["foobar".to_string(), "j+1".to_string()];

    println!("{}", optimize_line(&mut line, indices, sources, targets));
}

// Time complexity = O(l*n)
// Space complexity = O(l)
