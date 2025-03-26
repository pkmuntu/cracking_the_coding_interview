use std::cell::RefCell;
use std::rc::Rc;
#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct Node {
    pub val: String,
    pub left: Option<Rc<RefCell<Node>>>,
    pub right: Option<Rc<RefCell<Node>>>,
}

impl Node {
    pub fn new(val: String) -> Self {
        Node {
            val,
            left: None,
            right: None,
        }
    }
    pub fn insert(&mut self, val: String) {
        if val > self.val {
            match self.right {
                None => {
                    self.right = Some(Rc::new(RefCell::new(Node {
                        val: val.to_string(),
                        left: None,
                        right: None,
                    })))
                }
                Some(ref mut node) => node.borrow_mut().insert(val.to_string()),
            }
        } else {
            match self.left {
                None => {
                    self.left = Some(Rc::new(RefCell::new(Node {
                        val: val.to_string(),
                        left: None,
                        right: None,
                    })))
                }
                Some(ref mut node) => node.borrow_mut().insert(val.to_string()),
            }
        }
    }
}
#[derive(Debug, Default, PartialEq, Eq)]
pub struct BinarySearchTree {
    pub root: Node,
}
impl BinarySearchTree {
    pub fn new(val: String) -> Self {
        BinarySearchTree {
            root: Node::new(val.to_string()),
        }
    }
    pub fn insert(&mut self, val: String) {
        self.root.insert(val.to_string());
    }
}

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct Translator {}

impl Translator {
    pub fn new() -> Self {
        Translator {}
    }
    fn serialize(self, root: Option<Rc<RefCell<Node>>>, s: &mut String) {
        let mut res: Vec<String> = Vec::new();
        self.pre_order(root, &mut res);
        for i in 0..res.len() {
            s.push_str(&res[i]);
            if i + 1 < res.len() {
                s.push(',');
            }
        }
    }

    fn pre_order(&self, root: Option<Rc<RefCell<Node>>>, mut res: &mut Vec<String>) {
        if !root.clone().is_none() {
            res.push(root.clone().unwrap().borrow().val.to_string());
            self.pre_order(root.clone().unwrap().borrow().left.clone(), &mut res);
            self.pre_order(root.clone().unwrap().borrow().right.clone(), &mut res);
        }
    }

    fn deserialize(self, data: &mut String) -> Node {
        let mut root: Option<Node> = None;
        let lst = data.split(",");
        //let mut stack: Vec<Node> = Vec::new();
        for name in lst.into_iter() {
            if root.clone().is_none() {
                root = Some(Node::new(name.to_string()));
                // stack.push(root.clone().unwrap());
            } else {
                root.as_mut().unwrap().insert(name.to_string());
            }
        }
        return root.unwrap();
    }
}

fn main() {
    // Driver code
    let mut bst = BinarySearchTree::new("Jeanette".to_string());
    let names: Vec<String> = vec!["Elia", "Albert", "Latasha", "Elvira", "Kandice", "Maggie"]
        .into_iter()
        .map(String::from)
        .collect();
    for name in names.into_iter() {
        bst.insert(name.to_string());
    }
    println!("Original BST: ");
    printTree(Some(bst.root.clone()));
    println!();
    let mut res: Vec<String> = Vec::new();
    let mut display = String::new();
    let mut t = Translator::new();
    t.clone()
        .serialize(Some(Rc::new(RefCell::new(bst.root))), &mut display);
    println!("Serialized:  {:?}", display);
    let deserial = t.clone().deserialize(&mut display);
    println!();
    println!("Deserialized: ");
    printTree(Some(deserial));
}

// Method                    Time complexity        Space complexity
// serialize                     O(n)                   O(n)
// deserialzie                   O(n)                   O(n)
