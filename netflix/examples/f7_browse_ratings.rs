pub struct MaxStack {
    main_stack: Vec<i32>,
    maximum_stack: Vec<i32>,
}

impl MaxStack {
    pub fn new() -> Self {
        Self {
            main_stack: Vec::new(),
            maximum_stack: Vec::new(),
        }
    }

    pub fn pop(&mut self) {
        self.main_stack.pop();
        self.maximum_stack.pop();
    }

    pub fn push(&mut self, value: i32) {
        self.main_stack.push(value);
        if !self.maximum_stack.is_empty() && self.maximum_stack.last().unwrap() > &value {
            self.maximum_stack.push(*self.maximum_stack.last().unwrap());
        } else {
            self.maximum_stack.push(value);
        }
    }

    pub fn max_rating(&self) -> i32 {
        *self.maximum_stack.last().unwrap()
    }
}

fn main() {
    let mut stack = MaxStack::new();
    stack.push(5);
    stack.push(0);
    stack.push(2);
    stack.push(4);
    stack.push(6);
    stack.push(3);
    stack.push(10);

    print!("Maximum Rating: ");
    println!("{:}", stack.max_rating());

    stack.pop();
    println!("After clicking back button: ");
    print!("Maximum Rating: ");
    println!("{:}", stack.max_rating());
}

// Time complexity = O(1)
// Space complexity = O(n)
