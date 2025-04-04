use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TreeNode {
    pub val: String,
    pub children: Vec<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    pub fn new(val: String) -> Self {
        TreeNode {
            val,
            children: vec![],
        }
    }

    pub fn insert(val: String, root: Vec<Rc<RefCell<TreeNode>>>) -> Self {
        TreeNode {
            val: val,
            children: root,
        }
    }
}
fn serialize(root: Option<Rc<RefCell<TreeNode>>>) -> String {
    if root == None {
        return "".to_string();
    }
    let mut result: Vec<String> = Vec::new();
    let mut queue: VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::new();
    queue.push_back(root.unwrap());
    let mut childrenCount: Vec<i32> = Vec::new();
    while !queue.is_empty() {
        let size = queue.len();
        for s in 0..size {
            let curr = queue.pop_front().unwrap();
            childrenCount.push(curr.borrow().children.len() as i32);
            if Some(curr.clone()).is_none() {
                result.push("n".to_string());
                continue;
            }
            result.push(curr.borrow().val.to_string());
            for next in curr.borrow().children.to_vec().into_iter() {
                queue.push_back(next);
            }
        }
    }
    let delim = ",".to_string();
    let s: String = format!("{:?}{}{:?}", childrenCount, "-", result.join(&delim));
    return s;
}

fn deserialize(data: String) -> Option<Rc<RefCell<TreeNode>>> {
    if data == "" {
        return None;
    }
    let mut delim = "-";
    let mut childrenCount: Vec<i32> = Vec::new();
    let mut countAndResult: Vec<&str> = data.split(delim).collect();

    delim = ",";
    let mut tem: Vec<&str> = countAndResult[0].split(delim).collect();
    let mut temp: Vec<String> = vec!["".to_string(); tem.len()];
    for i in 0..tem.len() {
        temp[i] = tem[i].replace(&['[', ']', ' '][..], "").to_string();
    }
    let i = temp[0].parse::<i32>();
    for c in temp.into_iter() {
        let i: i32 = c.parse().unwrap_or(0);
        childrenCount.push(i);
    }

    let mut allNodes: Vec<&str> = countAndResult[1].split(delim).collect();
    let mut root = Rc::new(RefCell::new(TreeNode::insert(
        allNodes[0].to_string(),
        vec![],
    )));
    let mut i = 1;
    let mut allLevelCountIndex = 0;

    let mut queue: VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::new();
    queue.push_back(root.clone());

    while !queue.is_empty() {
        let size = queue.len();
        for s in 0..size {
            let mut curr = queue.pop_front().unwrap();
            for k in 0..childrenCount[allLevelCountIndex] {
                if !allNodes[i].is_empty() {
                    let next = Rc::new(RefCell::new(TreeNode::insert(
                        allNodes[i].to_string(),
                        vec![],
                    )));
                    curr.borrow_mut().children.push(next.clone());
                    queue.push_back(next);
                }
                i += 1;
            }
            allLevelCountIndex += 1;
        }
    }
    return Some(root.clone());
}

fn main() {
    let mut root: Vec<Rc<RefCell<TreeNode>>> = Vec::new();
    let mut rootD1: Vec<Rc<RefCell<TreeNode>>> = Vec::new();
    rootD1.push(Rc::new(RefCell::new(TreeNode::new("F3".to_string()))));
    rootD1.push(Rc::new(RefCell::new(TreeNode::new("F4".to_string()))));
    root.push(Rc::new(RefCell::new(TreeNode::insert(
        "D1".to_string(),
        rootD1,
    ))));
    root.push(Rc::new(RefCell::new(TreeNode::new("F1".to_string()))));
    root.push(Rc::new(RefCell::new(TreeNode::new("F2".to_string()))));
    let mut root1 = TreeNode::insert("r".to_string(), root);

    let s = serialize(Some(Rc::new(RefCell::new(root1))));
    println!("{}", s);
    println!("Deserializing the string:");
    let mut flag: Vec<bool> = vec![false; 6];
    //print!(deserialize(s), &mut flag, 0, false);
    println!("{:?}", deserialize(s));
}

//    Method             Time complexity          Space complexity
//    serialize             O(n)                     O(n)
//    deserialize           O(n)                     O(n)
