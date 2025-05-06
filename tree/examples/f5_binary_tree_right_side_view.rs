use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
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

pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut res = Vec::new();
    let mut curlevel = 0;
    if root.is_none() {
        return res;
    }

    let mut dq = VecDeque::new();
    dq.push_back((0, root.clone()));
    res.push(root.as_ref().unwrap().borrow().val);
    while !dq.is_empty() {
        if let Some((level, Some(node))) = dq.pop_front() {
            dq.push_back((level + 1, node.borrow().right.clone()));
            dq.push_back((level + 1, node.borrow().left.clone()));
            if level > curlevel {
                res.push(node.borrow().val);
                curlevel = level;
            }
        }
    }
    res
}

fn main() {
    let mut root = TreeNode::new(1);
    root.left = Some(Rc::new(RefCell::new(TreeNode::new(2))));
    root.right = Some(Rc::new(RefCell::new(TreeNode::new(3))));
    root.left.clone().unwrap().borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(4))));
    root.left.clone().unwrap().borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(5))));
    root.right.clone().unwrap().borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(6))));
    root.right.clone().unwrap().borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(7))));
    root.right
        .clone()
        .unwrap()
        .borrow_mut()
        .right
        .clone()
        .unwrap()
        .borrow_mut()
        .left = Some(Rc::new(RefCell::new(TreeNode::new(8))));

    let res = right_side_view(Some(Rc::new(RefCell::new(root))));
    println!("{:?}", res);
}

// Time complexity = O(N)
// Space complxity = O(H)
