use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: String,
    pub next: Option<Rc<RefCell<TreeNode>>>,
    pub children: Vec<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    pub fn new(val: String) -> Self {
        Self {
            val,
            next: None,
            children: vec![],
        }
    }
}

fn traversing_dom_tree(
    root: &mut Option<Rc<RefCell<TreeNode>>>,
    prev: &mut Option<Rc<RefCell<TreeNode>>>,
    leftmost: &mut Option<Rc<RefCell<TreeNode>>>,
) -> Option<Rc<RefCell<TreeNode>>> {
    match &root {
        Some(node) => {
            *leftmost = root.clone();

            // variable to keep track of nodes on the "current" level
            let mut curr = leftmost.clone();

            // Traverse till last node
            while !leftmost.is_none() {
                // "prev" tracks the latest node on the "next" level
                // "curr" tracks the latest node on the current level
                *prev = None;
                curr = leftmost.clone();

                *leftmost = None;

                while !curr.clone().is_none() {
                    println!("{:?}", curr.clone().unwrap().borrow().val);

                    // process all the children and update the prev
                    // and leftmost pointer as necessary.
                    for child in curr
                        .clone()
                        .unwrap()
                        .borrow_mut()
                        .children
                        .to_vec()
                        .into_iter()
                    {
                        // process the child
                        if !Some(child.clone()).is_none() {
                            if !prev.clone().is_none() {
                                prev.clone().unwrap().borrow_mut().next = Some(child.clone());
                            } else {
                                *leftmost = Some(child.clone());
                            }
                            *prev = Some(child.clone());
                        }
                    }
                    // Move the the next node.
                    curr = curr.unwrap().borrow().next.clone();
                }
            }
            return Some(node.clone());
        }
        None => {
            return root.clone();
        }
    };
}

fn main() {
    let mut root = TreeNode::new("body".to_string());
    let mut prev = None;
    let mut leftmost = None;
    root.children
        .push(Rc::new(RefCell::new(TreeNode::new("div".to_string()))));
    root.children
        .push(Rc::new(RefCell::new(TreeNode::new("h1".to_string()))));
    root.children
        .push(Rc::new(RefCell::new(TreeNode::new("div".to_string()))));
    root.children[0]
        .borrow_mut()
        .children
        .push(Rc::new(RefCell::new(TreeNode::new("h2".to_string()))));
    root.children[0].borrow_mut().children[0]
        .borrow_mut()
        .children
        .push(Rc::new(RefCell::new(TreeNode::new("ul".to_string()))));
    root.children[0]
        .borrow_mut()
        .children
        .push(Rc::new(RefCell::new(TreeNode::new("h3".to_string()))));
    root.children[0].borrow_mut().children[1]
        .borrow_mut()
        .children
        .push(Rc::new(RefCell::new(TreeNode::new("a".to_string()))));
    root.children[0].borrow_mut().children[1]
        .borrow_mut()
        .children
        .push(Rc::new(RefCell::new(TreeNode::new("p".to_string()))));
    root.children[0].borrow_mut().children[1]
        .borrow_mut()
        .children
        .push(Rc::new(RefCell::new(TreeNode::new("table".to_string()))));
    root.children[2]
        .borrow_mut()
        .children
        .push(Rc::new(RefCell::new(TreeNode::new("p".to_string()))));
    root.children[2]
        .borrow_mut()
        .children
        .push(Rc::new(RefCell::new(TreeNode::new("a".to_string()))));
    root.children[2]
        .borrow_mut()
        .children
        .push(Rc::new(RefCell::new(TreeNode::new("p".to_string()))));

    let res = traversing_dom_tree(
        &mut Some(Rc::new(RefCell::new(root))),
        &mut prev,
        &mut leftmost,
    );
}

// Time complexity = o(n)
// Space complexity = O(1)
