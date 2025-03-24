use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Node {
    pub prod: i32,
    pub next: Option<Rc<RefCell<Node>>>,
    pub related: Option<Rc<RefCell<Node>>>,
}

impl Node {
    pub fn new(
        prod: i32,
        next: Option<Rc<RefCell<Node>>>,
        related: Option<Rc<RefCell<Node>>>,
    ) -> Self {
        Node {
            prod: prod,
            next: next,
            related: related,
        }
    }
}

fn get_cloned_node(
    node: Option<Rc<RefCell<Node>>>,
    visited: &mut HashMap<i32, Rc<RefCell<Node>>>,
) -> Option<Rc<RefCell<Node>>> {
    if !node.is_none() {
        let prod = node.clone().unwrap().borrow().clone().prod;

        // Otherwise create a new node, add to the hashmap and return it
        return Some(
            visited
                .entry(prod)
                .or_insert(Rc::new(RefCell::new(Node::new(prod, None, None))))
                .clone(),
        );
    }
    return None;
}

fn copy_product_relations(
    mut head: &mut Option<Rc<RefCell<Node>>>,
    mut visited: &mut HashMap<i32, Rc<RefCell<Node>>>,
) -> Option<Rc<RefCell<Node>>> {
    if head.is_none() {
        return head.clone();
    }

    let mut old_node = &mut head.clone();

    // Creating the new head node.
    let mut new_node = Some(Rc::new(RefCell::new(Node::new(
        old_node.clone().unwrap().borrow_mut().prod,
        None,
        None,
    ))));
    visited
        .entry(old_node.clone().unwrap().borrow_mut().prod)
        .or_insert(new_node.as_ref().unwrap().clone());

    // Iterate on the linked list until all nodes are cloned.
    while !old_node.clone().is_none() {
        // Get the clones of the nodes referenced by related and next pointers.
        new_node.as_ref().unwrap().borrow_mut().related = get_cloned_node(
            old_node.clone().unwrap().borrow_mut().related.clone(),
            &mut visited,
        );
        new_node.as_ref().unwrap().borrow_mut().next = get_cloned_node(
            old_node.clone().unwrap().borrow_mut().next.clone(),
            &mut visited,
        );
        *old_node = old_node.clone().unwrap().borrow_mut().next.clone();
        if !new_node.clone().is_none() {
            if !new_node.clone().unwrap().borrow_mut().next.is_none() {
                new_node = new_node.unwrap().borrow_mut().clone().next;
            }
        }
    }
    return Some(
        visited
            .get(&head.clone().unwrap().borrow().prod)
            .unwrap()
            .clone(),
    );
}

fn main() {
    // Visited hashmap
    let mut visited: HashMap<i32, Rc<RefCell<Node>>> = HashMap::new();
    let mut products = to_list(vec![3, 1, 5, 4], vec![2, 0, -1, 1]);

    // // The to_list(values, pointer) is a utility function with parameters as:
    // // 1. values: an array of values to be stored in linked list, i.e., product IDs.
    // // 2. pointer: an array containing indices of values that the "related" pointer
    // // of the corresponding product will point to.
    // // This function creates the list and returns the head.

    println!("Original list:");
    println!("{}", listToString(&mut products.clone()));
    // The listToString(head) function is also a utility function that returns
    // string representation of the list.

    let mut copied_list = copy_product_relations(&mut products, &mut visited);
    //println!("end:  {:?}",copiedList);
    println!("Deep copy of list: ");
    println!("{}", listToString(&mut copied_list));
}

// Time complexity = O(n)
// Space complexity = O(n)
