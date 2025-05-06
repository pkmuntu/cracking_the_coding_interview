use std::cell::RefCell;
use std::cmp;
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

fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut max = i32::min_value();
    max_contrib(root.as_ref(), &mut max);
    max
}

fn max_contrib(root: Option<&Rc<RefCell<TreeNode>>>, max: &mut i32) -> i32 {
    if let Some(node) = root {
        let left = max_contrib(node.borrow().left.as_ref(), max);
        let right = max_contrib(node.borrow().right.as_ref(), max);
        *max = cmp::max(
            node.borrow().val + cmp::max(left, 0) + cmp::max(right, 0),
            *max,
        );
        node.borrow().val + cmp::max(cmp::max(left, right), 0)
    } else {
        0
    }
}

fn main() {
    let mut root = TreeNode::new(-8);
    root.left = Some(Rc::new(RefCell::new(TreeNode::new(2))));
    root.right = Some(Rc::new(RefCell::new(TreeNode::new(17))));
    root.left.clone().unwrap().borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    root.left.clone().unwrap().borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(4))));
    root.right.clone().unwrap().borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(19))));
    root.right.clone().unwrap().borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(5))));

    let res = max_path_sum(Some(Rc::new(RefCell::new(root))));
    println!("{:}", res);
}

// Time complexity = O(N)
// Space complexity = O(H)
