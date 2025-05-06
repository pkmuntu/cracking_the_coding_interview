use std::cell::RefCell;
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

fn leftboundry(root: Option<Rc<RefCell<TreeNode>>>, nodes: &mut Vec<i32>) {
    if let Some(node) = root {
        let node = node.borrow();
        let left = &node.left;
        let right = &node.right;
        if left.is_some() || right.is_some() {
            nodes.push(node.val);
        }
        if left.is_some() {
            leftboundry(left.clone(), nodes);
        } else {
            leftboundry(right.clone(), nodes);
        }
    }
}
fn leaves(root: Option<Rc<RefCell<TreeNode>>>, nodes: &mut Vec<i32>) {
    if let Some(node) = root {
        let node = node.borrow();
        let left = &node.left;
        let right = &node.right;
        if left.is_none() && right.is_none() {
            nodes.push(node.val);
        } else {
            leaves(left.clone(), nodes);
            leaves(right.clone(), nodes);
        }
    }
}
fn rightboundry(root: Option<Rc<RefCell<TreeNode>>>, nodes: &mut Vec<i32>) {
    if let Some(node) = root {
        let node = node.borrow();
        let left = &node.left;
        let right = &node.right;
        if right.is_some() {
            rightboundry(right.clone(), nodes);
        } else {
            rightboundry(left.clone(), nodes);
        }
        if left.is_some() || right.is_some() {
            nodes.push(node.val);
        }
    }
}

fn boundary(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut res: Vec<i32> = vec![];
    if let Some(node) = root {
        let node = node.borrow();
        res.push(node.val);
        leftboundry(node.left.clone(), &mut res);
        leaves(node.left.clone(), &mut res);
        leaves(node.right.clone(), &mut res);
        rightboundry(node.right.clone(), &mut res);
    } else {
        return vec![];
    }
    res
}
fn main() {
    let mut root = TreeNode::new(1);
    root.left = Some(Rc::new(RefCell::new(TreeNode::new(2))));
    root.right = Some(Rc::new(RefCell::new(TreeNode::new(6))));
    root.left.clone().unwrap().borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(3))));
    root.left.clone().unwrap().borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(4))));
    root.right.clone().unwrap().borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(7))));
    root.right.clone().unwrap().borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(8))));
    root.left
        .clone()
        .unwrap()
        .borrow_mut()
        .right
        .clone()
        .unwrap()
        .borrow_mut()
        .left = Some(Rc::new(RefCell::new(TreeNode::new(5))));
    root.right
        .clone()
        .unwrap()
        .borrow_mut()
        .left
        .clone()
        .unwrap()
        .borrow_mut()
        .right = Some(Rc::new(RefCell::new(TreeNode::new(9))));
    root.right
        .clone()
        .unwrap()
        .borrow_mut()
        .right
        .clone()
        .unwrap()
        .borrow_mut()
        .right = Some(Rc::new(RefCell::new(TreeNode::new(10))));

    let res = boundary(Some(Rc::new(RefCell::new(root))));
    println!("{:?}", res);
}

// Time complexity = O(N)
// Space complexity = O(N)
