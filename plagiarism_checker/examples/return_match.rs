fn _match(cheater: String, student: String) -> String {
    let mut window: String = "".to_string();
    let mut j: i32 = 0;
    let mut min = cheater.len() + 1;
    let mut i: i32 = 0;
    while i < cheater.len() as i32 {
        if cheater.chars().nth(i as usize).unwrap() == student.chars().nth(j as usize).unwrap() {
            j += 1;
            if j == student.len() as i32 {
                let end = i + 1;
                j = j - 1;
                while j >= 0 {
                    if cheater.chars().nth(i as usize).unwrap()
                        == student.chars().nth(j as usize).unwrap()
                    {
                        j = j - 1;
                    }
                    i = i - 1;
                }
                j += 1;
                i += 1;

                if ((end - i) as usize) < min {
                    min = (end - i) as usize;
                    window = cheater[(i as usize)..].to_string();
                }
            }
        }
        i = i + 1;
    }
    return window;
}

fn main() {
    // Driver code

    let cheater: String = "quiqutit".to_string();
    let student: String = "quit".to_string();

    println!("{}{}", "Copied Content: ", _match(cheater, student));
}
