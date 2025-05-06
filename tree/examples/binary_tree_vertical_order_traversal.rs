use std::cell::RefCell;
use std::cmp;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    val: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    pub fn new(val: i32) -> Self {
        Self {
            val,
            left: None,
            right: None,
        }
    }
}

fn vertical_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    let mut output: Vec<Vec<i32>> = Vec::new();
    let mut nodes_list: HashMap<i32, Vec<i32>> = HashMap::new();

    if root.is_none() {
        return output;
    }
    let mut queue = VecDeque::new();
    let mut column = 0;
    queue.push_back((root.clone(), column));

    let mut min_column = 0;
    let mut max_column = 0;

    while !queue.is_empty() {
        if let Some((Some(node), level)) = queue.pop_front() {
            column = level;
            nodes_list
                .entry(column)
                .or_insert(Vec::new())
                .push(node.borrow().val);
            min_column = cmp::min(min_column, column);
            max_column = cmp::max(max_column, column);
            queue.push_front((node.borrow().left.clone(), column - 1));
            queue.push_front((node.borrow().right.clone(), column + 1));
        }
    }

    for i in min_column..max_column + 1 {
        output.push(nodes_list[&i].clone());
    }

    return output;
}

fn main() {
    let mut root = TreeNode::new(-8);
    root.left = Some(Rc::new(RefCell::new(TreeNode::new(2))));
    root.right = Some(Rc::new(RefCell::new(TreeNode::new(17))));
    root.left.clone().unwrap().borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    root.left.clone().unwrap().borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(4))));
    root.right.clone().unwrap().borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(19))));
    root.right.clone().unwrap().borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(5))));
    // root.left.clone().unwrap().borrow_mut().right.clone().unwrap().borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(7))));
    // root.left.clone().unwrap().borrow_mut().right.clone().unwrap().borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(8))));

    let res = vertical_order(Some(Rc::new(RefCell::new(root))));
    println!("{:?}", res);
}

// Time complexity = O(N)
// Space complexity = O(N)
