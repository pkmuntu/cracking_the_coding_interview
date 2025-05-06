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
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}
fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    let mut res = Vec::new();
    let mut curlevel = 0;
    if root.is_none() {
        return res;
    }
    let mut dq = VecDeque::new();
    dq.push_back((0, root.clone()));
    let mut vec = Vec::new();
    while !dq.is_empty() {
        if let Some((level, Some(node))) = dq.pop_front() {
            dq.push_back((level + 1, node.borrow().left.clone()));
            dq.push_back((level + 1, node.borrow().right.clone()));
            if level > curlevel {
                if curlevel % 2 == 1 {
                    vec.reverse();
                }
                res.push(vec.clone());
                vec.clear();
                curlevel = level;
            }
            vec.push(node.borrow().val);
        }
    }
    if !vec.is_empty() {
        if curlevel % 2 == 1 {
            vec.reverse();
        }
        res.push(vec)
    }
    res
}

fn main() {
    let mut root = TreeNode::new(3);
    root.left = Some(Rc::new(RefCell::new(TreeNode::new(6))));
    root.right = Some(Rc::new(RefCell::new(TreeNode::new(9))));
    root.right.clone().unwrap().borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(15))));
    root.right.clone().unwrap().borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(12))));

    let res = zigzag_level_order(Some(Rc::new(RefCell::new(root))));
    println!("{:?}", res);
}

// Time complexity = O(N)
// Space complexity = O(N)
