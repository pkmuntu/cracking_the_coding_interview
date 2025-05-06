use std::cell::RefCell;
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
fn flipEquiv(root: Option<Rc<RefCell<TreeNode>>>, root1: Option<Rc<RefCell<TreeNode>>>) -> bool {
    if let (Some(node1), Some(node2)) = (&root, &root1) {
        let val = node1.borrow().val;
        let val1 = node2.borrow().val;
        let left = &node1.borrow().left;
        let right = &node1.borrow().right;
        let left1 = &node2.borrow().left;
        let right1 = &node2.borrow().right;
        val == val1
            && (flipEquiv(left.clone(), left1.clone()) && flipEquiv(right.clone(), right1.clone())
                || flipEquiv(left.clone(), right1.clone())
                    && flipEquiv(right.clone(), left1.clone()))
    } else {
        root == root1
    }
}

fn main() {
    let mut root = TreeNode::new(0);
    let mut root1 = TreeNode::new(0);
    let res = flipEquiv(
        Some(Rc::new(RefCell::new(root))),
        Some(Rc::new(RefCell::new(root1))),
    );
    println!("{:}", res);

    let mut root = TreeNode::new(1);
    let mut root1 = TreeNode::new(1);
    root.left = Some(Rc::new(RefCell::new(TreeNode::new(2))));
    root.right = Some(Rc::new(RefCell::new(TreeNode::new(3))));
    root.left.clone().unwrap().borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(4))));
    root.left.clone().unwrap().borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(5))));
    root.right.clone().unwrap().borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(6))));
    root.right.clone().unwrap().borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(9))));
    root.left
        .clone()
        .unwrap()
        .borrow_mut()
        .right
        .clone()
        .unwrap()
        .borrow_mut()
        .left = Some(Rc::new(RefCell::new(TreeNode::new(7))));
    root.left
        .clone()
        .unwrap()
        .borrow_mut()
        .right
        .clone()
        .unwrap()
        .borrow_mut()
        .right = Some(Rc::new(RefCell::new(TreeNode::new(8))));

    root1.left = Some(Rc::new(RefCell::new(TreeNode::new(3))));
    root1.right = Some(Rc::new(RefCell::new(TreeNode::new(2))));
    root1.left.clone().unwrap().borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(9))));
    root1.left.clone().unwrap().borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(6))));
    root1.right.clone().unwrap().borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(4))));
    root1.right.clone().unwrap().borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(5))));
    root1
        .right
        .clone()
        .unwrap()
        .borrow_mut()
        .right
        .clone()
        .unwrap()
        .borrow_mut()
        .left = Some(Rc::new(RefCell::new(TreeNode::new(8))));
    root1
        .right
        .clone()
        .unwrap()
        .borrow_mut()
        .right
        .clone()
        .unwrap()
        .borrow_mut()
        .right = Some(Rc::new(RefCell::new(TreeNode::new(7))));

    let res = flipEquiv(
        Some(Rc::new(RefCell::new(root))),
        Some(Rc::new(RefCell::new(root1))),
    );
    println!("{:}", res);
}

// Time compllexity = O(min(N1, N2))
// Space complexity = O(min(N1, N2))
