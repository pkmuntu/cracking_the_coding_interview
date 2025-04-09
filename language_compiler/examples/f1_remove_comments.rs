fn remove_comments(source: &Vec<String>, output: &mut Vec<String>) {
    let mut buffer = String::from("");
    let mut block = false;
    for line in source.into_iter() {
        let mut i = 0;
        while i < line.len() {
            let c = line.chars().nth(i).unwrap();
            if c == '/' && (i + 1) < line.len() && line.chars().nth(i + 1).unwrap() == '/' && !block
            {
                i = line.len();
            } else if c == '/'
                && (i + 1) < line.len()
                && line.chars().nth(i + 1).unwrap() == '*'
                && !block
            {
                block = true;
                i += 1;
            } else if c == '*'
                && (i + 1) < line.len()
                && line.chars().nth(i + 1).unwrap() == '/'
                && block
            {
                block = false;
                i += 1;
            }
            //normal chacter -> Append to buffer if not in block comment.
            else if !block {
                buffer.push(c);
            }
            i += 1;
        }
        if buffer != "" && !block {
            output.push(buffer);
            buffer = "".to_string();
        }
    }
}

fn main() {
    let source: Vec<String> = vec![
        "/* Example code for feature */",
        "int main() {",
        "  /*",
        "  This is a",
        "  block comment",
        "  */",
        "  int value = 10;  // This is an inline comment",
        "  int sum = value + /* this is // also a block */ value;",
        "  return 0;",
        "}",
    ]
    .into_iter()
    .map(String::from)
    .collect();
    let mut output: Vec<String> = Vec::new();
    remove_comments(&source, &mut output);
    println!("{:?}", output);
}

// Time complexity = O(n)
// Space complexity = O(n)
