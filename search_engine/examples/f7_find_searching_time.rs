fn service_time(n: i32, logs: Vec<String>) -> Vec<i32> {
    let mut stack: Vec<i32> = Vec::new();
    let mut func_times: Vec<i32> = vec![0; n as usize];
    let mut serv: Vec<&str> = logs[0].split(':').collect();
    stack.push(serv[0].parse::<i32>().unwrap());
    let mut time = serv[2].parse::<i32>().unwrap();
    for i in 0..logs.len() {
        serv = logs[i].split(':').collect();

        let last = stack.len() as i32 - 1;
        if serv[1] == "start" {
            if !stack.is_empty() {
                func_times[stack[last as usize] as usize] += serv[2].parse::<i32>().unwrap() - time;
            }
            stack.push(serv[0].parse::<i32>().unwrap());
            time = serv[2].parse::<i32>().unwrap();
        } else {
            func_times[stack[last as usize] as usize] += serv[2].parse::<i32>().unwrap() - time + 1;
            stack.pop();
            time = serv[2].parse::<i32>().unwrap() + 1;
        }
    }

    return func_times;
}
fn main() {
    // Driver code
    let logs: Vec<String> = vec![
        "0:start:0".to_string(),
        "0:start:2".to_string(),
        "0:end:5".to_string(),
        "1:start:6".to_string(),
        "1:end:6".to_string(),
        "0:end:7".to_string(),
    ];
    let result = service_time(2, logs);

    println!("{:?}", result);
}

// Time complexity = O(n)
// Space complexity = O(n)
