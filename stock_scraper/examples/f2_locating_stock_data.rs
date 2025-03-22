use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::HashSet;
use std::rc::Rc;

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone)]
pub struct TreeNode {
    pub val: i32,
    pub children: Vec<TreeNode>,
}

impl TreeNode {
    pub fn new(val: i32) -> Self {
        Self {
            val,
            children: vec![],
        }
    }
}

fn lowest_common_ancestor(
    root: Option<Rc<RefCell<TreeNode>>>,
    a: &mut Option<TreeNode>,
    b: &mut Option<TreeNode>,
) -> i32 {
    let mut stack: Vec<TreeNode> = Vec::new();
    let mut parent: HashMap<TreeNode, TreeNode> = HashMap::new();
    let node = &root;
    parent
        .entry(node.as_ref().unwrap().borrow_mut().clone())
        .or_default();
    stack.push(node.clone().unwrap().borrow_mut().clone());

    while !parent.contains_key(&a.clone().unwrap()) || !parent.contains_key(&b.clone().unwrap()) {
        let node1 = stack.pop().unwrap();

        // save the parent pointer while iterating
        for child in node1.clone().children.to_vec().into_iter() {
            parent.entry(child.clone()).or_insert(node1.clone());
            stack.push(child.clone());
        }
    }

    let mut ancestors: HashSet<TreeNode> = HashSet::new();
    while !a.is_none() {
        ancestors.insert(a.clone().unwrap());
        if parent.contains_key(&a.clone().unwrap()) {
            *a = Some(parent.clone().get_mut(&a.clone().unwrap()).unwrap().clone());
        } else {
            *a = None;
        }
    }

    while !ancestors.contains(&b.clone().unwrap()) {
        if parent.contains_key(&b.clone().unwrap()) {
            *b = Some(parent.clone().get_mut(&b.clone().unwrap()).unwrap().clone());
        } else {
            break;
        }
    }
    return b.clone().unwrap().val;
}

fn main() {
    let mut root = TreeNode::new(1);

    root.children.push(TreeNode::new(2));
    root.children.push(TreeNode::new(3));
    root.children.push(TreeNode::new(4));
    root.children[0].children.push(TreeNode::new(5));
    root.children[0].children[0]
        .children
        .push(TreeNode::new(10));
    root.children[0].children.push(TreeNode::new(6));
    root.children[0].children[1]
        .children
        .push(TreeNode::new(11));
    root.children[0].children[1]
        .children
        .push(TreeNode::new(12));
    root.children[0].children[1]
        .children
        .push(TreeNode::new(13));
    root.children[2].children.push(TreeNode::new(7));
    root.children[2].children.push(TreeNode::new(8));
    root.children[2].children.push(TreeNode::new(9));

    let a = root.children[0].children[1].children[2].clone();
    let b = root.children[0].children[0].children[0].clone();
    let lca = lowest_common_ancestor(
        Some(Rc::new(RefCell::new(root))),
        &mut Some(a.clone()),
        &mut Some(b.clone()),
    );
    println!(
        "{}{:}{}{}{}{:}{}{}{}{:}{}",
        "\"",
        lca,
        "\"",
        " is the lowest common ancestor of the nodes ",
        "\"",
        a.val,
        "\"",
        " and ",
        "\"",
        b.val,
        "\""
    );
}

// Time complexity = O(n)
// Space complexity = O(n)
