static mut PROCESS: Vec<String> = Vec::new();
fn dfs(nums: Vec<f64>, target: i32) -> bool {
    if nums.len() == 1 {
        return (nums[0] - target as f64).abs() <= 0.001;
    }

    for i in 0..nums.len() {
        for j in 0..nums.len() {
            if i == j {
                continue;
            }

            let mut next_nums: Vec<f64> = Vec::new();
            for k in 0..nums.len() {
                if k == j || k == i {
                    continue;
                }

                next_nums.push(nums[k]);
            }

            let num1 = nums[i];
            let num2 = nums[j];

            // + and * are transitive => a * b = b * a;
            if i < j {
                next_nums.push(num1 + num2);
                let s: String = format!("{:}{}{:}{}{:}", num1, "+", num2, "=", num1 + num2);
                unsafe {
                    PROCESS.push(s);
                }
                if dfs(next_nums.to_vec(), target) {
                    return true;
                }

                // Back track
                next_nums.pop();
                unsafe {
                    PROCESS.pop();
                }

                next_nums.push(num1 * num2);
                let s: String = format!("{:}{}{:}{}{:}", num1, "*", num2, "=", num1 * num2);
                unsafe {
                    PROCESS.push(s);
                }

                if dfs(next_nums.to_vec(), target) {
                    return true;
                }

                // Back track
                next_nums.pop();
                unsafe {
                    PROCESS.pop();
                }
            }

            next_nums.push(num1 - num2);
            let s: String = format!("{:}{}{:}{}{:}", num1, "-", num2, "=", num1 - num2);
            unsafe {
                PROCESS.push(s);
            }
            if dfs(next_nums.to_vec(), target) {
                return true;
            }

            // Back track
            next_nums.pop();
            unsafe {
                PROCESS.pop();
            }

            next_nums.push(num1 / num2);
            let s: String = format!("{:}{}{:}{}{:}", num1, "/", num2, "=", num1 / num2);
            unsafe {
                PROCESS.push(s);
            }
            if dfs(next_nums.to_vec(), target) {
                return true;
            }

            // Back track
            next_nums.pop();
            unsafe {
                PROCESS.pop();
            }
        }
    }
    return false;
}

fn decode_message(nums: Vec<i32>, target: i32) -> bool {
    if nums.len() == 0 {
        return false;
    }
    let mut temp: Vec<f64> = Vec::new();
    for n in nums.into_iter() {
        temp.push(n as f64 * 1.0);
    }
    return dfs(temp, target);
}

fn main() {
    // Driver Code
    let message: Vec<i32> = vec![1, 5, 5, 8];
    let mdigest = 30;
    println!("{:}", decode_message(message, mdigest));

    println!("Here are the steps:");
    unsafe {
        println!("{}", PROCESS[0]);
        println!("{}", PROCESS[1]);
        println!("{}", PROCESS[2]);
    }
}
