use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq, Clone)]
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

pub fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    if root.is_none() {
        return None;
    }
    let mut curr = Rc::clone(root.as_ref().unwrap());
    while curr.borrow().left.is_some() || curr.borrow().right.is_some() {
        if curr.borrow().left.is_some() {
            let right = curr.borrow_mut().right.take();
            let left = curr.borrow_mut().left.take();
            let mut rmax = Rc::clone(&left.clone().unwrap());
            while rmax.borrow().right.is_some() {
                let x = Rc::clone(rmax.borrow().right.as_ref().unwrap());
                rmax = x;
            }
            rmax.borrow_mut().right = right;
            curr.borrow_mut().right = left;
            curr.borrow_mut().left = None;
        }
        if curr.borrow().right.is_some() {
            let next = Rc::clone(curr.borrow().right.as_ref().unwrap());
            curr = next;
        } else {
            break;
        }
    }
    return root.clone();
}

fn main() {
    let mut root = TreeNode::new(1);
    root.left = Some(Rc::new(RefCell::new(TreeNode::new(2))));
    root.right = Some(Rc::new(RefCell::new(TreeNode::new(4))));
    root.left.clone().unwrap().borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(3))));
    root.right.clone().unwrap().borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(5))));
    root.right.clone().unwrap().borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(6))));
    let mut res = flatten(&mut Some(Rc::new(RefCell::new(root.clone()))));
    println!("Tree: ");
    print(&mut res, &mut 10);
}

// Time complexity = O(n)
// Space complexity = O(1)
