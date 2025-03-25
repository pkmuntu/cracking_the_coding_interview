use std::rc::Rc;
use std::cell::RefCell;
#[derive(Debug, Default, PartialEq, Eq,Clone)]
pub struct Node {
  pub val: i32,
  pub left: Option<Box<Node>>,
  pub right: Option<Box<Node>>
}

impl Node {
  pub fn new(val: i32) ->Self { 
    Node {
      val,
      left: None,
      right: None
    }
  }
  pub fn insert(&mut self, val: i32) {
		if val > self.val {
			match self.right {
				None => self.right = Some(Box::new(Node {val:val, left:None, right:None})),
				Some(ref mut node) => node.insert(val)
			}
		}
		else {
			match self.left {
				None => self.left = Some(Box::new(Node {val:val, left:None, right:None})),
				Some(ref mut node) => node.insert(val)
			}
		}
	}
}
#[derive(Debug,Default, PartialEq, Eq)]
pub struct BinarySearchTree{
     pub root: Node
}
impl  BinarySearchTree{
  pub fn new(val: i32) ->Self { 
    BinarySearchTree {
       root: Node::new(val)
    }
  }
  pub fn insert(&mut self, val: i32) {
    self.root.insert(val);
	}
}
fn preOrder(node: Option<Box<Node>>, low: i32, high: i32,mut  output: &mut Vec<i32>){
    if !node.is_none() {
        if node.as_ref().unwrap().val <= high && low <= node.as_ref().unwrap().val
            {output.push(node.as_ref().unwrap().val);}
        if low <= node.as_ref().unwrap().val
            {preOrder(node.as_ref().unwrap().left.clone(), low, high,&mut output);}
        if node.as_ref().unwrap().val <= high
            {preOrder(node.as_ref().unwrap().right.clone(), low, high, &mut output);}
    }
}

fn productsInRange(root: Node, low: i32, high: i32)-> Vec<i32>{
    let mut output: Vec<i32> = Vec::new();
    preOrder(Some(Box::new(root)), low, high,&mut output);
    return output;
}


fn main() {
    // Driver code
    let mut bst = BinarySearchTree::new(9);

     bst.insert(6);
    bst.insert(14);
    bst.insert(20);
    bst.insert(1);
    bst.insert(30);
    bst.insert(8);
    bst.insert(17);
    bst.insert(5);
    let result = productsInRange(bst.root, 7, 20);
    println!("{:?}",result);
}

// Time complexity = O(n)
// Space complexity = O(n)
