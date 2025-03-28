use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::HashSet;
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

fn dfs(
    parent: Option<Rc<RefCell<TreeNode>>>,
    child: Option<Rc<RefCell<TreeNode>>>,
    mut neighbors: &mut HashMap<i32, Vec<i32>>,
) {
    // DFS to build adjacency list
    if child.is_none() {
        return;
    }
    if !parent.is_none() {
        if !neighbors.contains_key(&parent.clone().unwrap().borrow().val) {
            neighbors
                .entry(parent.clone().unwrap().borrow().val)
                .or_insert(vec![]);
        }
        if !neighbors.contains_key(&child.clone().unwrap().borrow().val) {
            neighbors
                .entry(child.clone().unwrap().borrow().val)
                .or_insert(vec![]);
        }

        neighbors
            .get_mut(&parent.clone().unwrap().borrow().val)
            .unwrap()
            .push(child.clone().unwrap().borrow().val);
        neighbors
            .get_mut(&child.clone().unwrap().borrow().val)
            .unwrap()
            .push(parent.clone().unwrap().borrow().val);
    }
    for i in 0..child.clone().unwrap().borrow().children.len() {
        dfs(
            child.clone(),
            Some(child.clone().unwrap().borrow().children[i].clone()),
            &mut neighbors,
        );
    }
}

fn get_devices(
    root: Option<Rc<RefCell<TreeNode>>>,
    server: Option<Rc<RefCell<TreeNode>>>,
    ttl: i32,
    bfs: &mut Vec<i32>,
) {
    let mut neighbors: HashMap<i32, Vec<i32>> = HashMap::new(); // Adjacency list
    dfs(None, root, &mut neighbors);

    // BFS to find nodes
    bfs.push(server.unwrap().borrow().val);
    let mut lookup: HashSet<i32> = bfs.iter().cloned().collect();
    for _i in 0..ttl {
        let mut temp: Vec<i32> = Vec::new();
        for node in bfs.into_iter() {
            for nei in neighbors.get(node).unwrap().into_iter() {
                if !lookup.contains(nei) {
                    temp.push(*nei);
                }
            }
        }
        *bfs = temp;
        for i in bfs.into_iter() {
            lookup.insert(*i);
        }
    }
}

fn main() {
    let mut root = TreeNode::new(1);
    root.children.push(Rc::new(RefCell::new(TreeNode::new(2))));
    root.children.push(Rc::new(RefCell::new(TreeNode::new(3))));
    root.children.push(Rc::new(RefCell::new(TreeNode::new(4))));
    root.children[0]
        .borrow_mut()
        .children
        .push(Rc::new(RefCell::new(TreeNode::new(5))));
    root.children[0].borrow_mut().children[0]
        .borrow_mut()
        .children
        .push(Rc::new(RefCell::new(TreeNode::new(10))));
    root.children[0]
        .borrow_mut()
        .children
        .push(Rc::new(RefCell::new(TreeNode::new(6))));
    root.children[0].borrow_mut().children[1]
        .borrow_mut()
        .children
        .push(Rc::new(RefCell::new(TreeNode::new(11))));
    root.children[0].borrow_mut().children[1]
        .borrow_mut()
        .children
        .push(Rc::new(RefCell::new(TreeNode::new(12))));
    root.children[0].borrow_mut().children[1]
        .borrow_mut()
        .children
        .push(Rc::new(RefCell::new(TreeNode::new(13))));
    root.children[2]
        .borrow_mut()
        .children
        .push(Rc::new(RefCell::new(TreeNode::new(7))));
    root.children[2]
        .borrow_mut()
        .children
        .push(Rc::new(RefCell::new(TreeNode::new(8))));
    root.children[2]
        .borrow_mut()
        .children
        .push(Rc::new(RefCell::new(TreeNode::new(9))));

    let server = root.children[0].borrow_mut().children[1].clone();
    let ttl = 2;
    let mut res: Vec<i32> = Vec::new();
    get_devices(
        Some(Rc::new(RefCell::new(root))),
        Some(server),
        ttl,
        &mut res,
    );
    println!("{}{:?}", "The TTL value will expire on node IDs: ", res);
}

// Time complexity = O(n)
// Space complexity = O(n)
