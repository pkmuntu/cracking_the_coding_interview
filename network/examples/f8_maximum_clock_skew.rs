use std::cell::RefCell;
use std::cmp;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub children: Vec<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            children: vec![],
        }
    }
}

static mut maxDiff: i32 = 5;

fn dfs(root: Option<Rc<RefCell<TreeNode>>>, mut maxi_val: i32, mut mini_val: i32) {
    match &root {
        Some(node) => {
            let possiblemaxDiff = cmp::max(
                (maxi_val - node.borrow().val).abs(),
                (mini_val - node.borrow().val).abs(),
            );
            unsafe {
                maxDiff = cmp::max(maxDiff, possiblemaxDiff);
            }
            maxi_val = cmp::max(maxi_val, node.borrow().val);
            mini_val = cmp::min(mini_val, node.borrow().val);
            for child in node.borrow().children.clone().into_iter() {
                dfs(Some(child), maxi_val, mini_val);
            }
            return;
        }
        None => {
            return;
        }
    };
}

fn max_clock_skew(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    match &root {
        Some(node) => {
            unsafe {
                maxDiff = 0;
            }
            dfs(Some(node.clone()), node.borrow().val, node.borrow().val);
            unsafe {
                return maxDiff;
            }
        }
        None => {
            return 0;
        }
    };
}

fn main() {
    let mut root = TreeNode::new(8);
    root.children.push(Rc::new(RefCell::new(TreeNode::new(3))));
    root.children.push(Rc::new(RefCell::new(TreeNode::new(10))));
    root.children.push(Rc::new(RefCell::new(TreeNode::new(12))));
    root.children[0]
        .borrow_mut()
        .children
        .push(Rc::new(RefCell::new(TreeNode::new(6))));
    root.children[0].borrow_mut().children[0]
        .borrow_mut()
        .children
        .push(Rc::new(RefCell::new(TreeNode::new(1))));
    root.children[0]
        .borrow_mut()
        .children
        .push(Rc::new(RefCell::new(TreeNode::new(5))));
    root.children[0].borrow_mut().children[1]
        .borrow_mut()
        .children
        .push(Rc::new(RefCell::new(TreeNode::new(2))));
    root.children[0].borrow_mut().children[1]
        .borrow_mut()
        .children
        .push(Rc::new(RefCell::new(TreeNode::new(3))));
    root.children[0].borrow_mut().children[1]
        .borrow_mut()
        .children
        .push(Rc::new(RefCell::new(TreeNode::new(4))));
    root.children[2]
        .borrow_mut()
        .children
        .push(Rc::new(RefCell::new(TreeNode::new(8))));
    root.children[2]
        .borrow_mut()
        .children
        .push(Rc::new(RefCell::new(TreeNode::new(7))));
    root.children[2]
        .borrow_mut()
        .children
        .push(Rc::new(RefCell::new(TreeNode::new(9))));

    let res = max_clock_skew(Some(Rc::new(RefCell::new(root))));
    println!(
        "{}{:}{}",
        "The maximum clock skew we'll encounter is: ", res, " seconds"
    );
}

// Time complexity = O(n)
// Space complexity = O(n)
