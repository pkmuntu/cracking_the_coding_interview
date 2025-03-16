use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Display;
use std::rc::Rc;

#[derive(Debug, Default, Clone)]
pub struct Node<K, V> {
    pub key: K,
    pub value: V,
    prev: Option<Rc<RefCell<Node<K, V>>>>,
    next: Option<Rc<RefCell<Node<K, V>>>>,
}

impl<K, V> Node<K, V> {
    fn new(key: K, value: V) -> Self {
        Self {
            key,
            value,
            prev: None,
            next: None,
        }
    }
}

#[derive(Default, Debug)]
pub struct LinkedList<K, V> {
    pub head: Option<Rc<RefCell<Node<K, V>>>>,
    pub tail: Option<Rc<RefCell<Node<K, V>>>>,
}

impl<K: Default, V: Default> LinkedList<K, V> {
    fn new() -> Self {
        Default::default()
    }

    fn move_to_tail(&mut self, node: &Rc<RefCell<Node<K, V>>>) {
        let next = node.borrow().next.as_ref().map(|a| a.clone());
        let prev = node.borrow().prev.as_ref().map(|a| a.clone());

        match (&prev, &next) {
            (None, None) => {
                // There is only one element
            }
            (Some(_), None) => {
                // Already present in last of linked list
            }
            (None, Some(next)) => {
                // present the node head of linked list
                node.borrow_mut().next = None;
                node.borrow_mut().prev = None;
                next.borrow_mut().prev = None;

                //  move the node to last
                let prev_tail = self.tail.as_ref().unwrap();
                prev_tail.borrow_mut().next = Some(Rc::clone(node));
                node.borrow_mut().prev = Some(Rc::clone(prev_tail));
                self.tail = Some(Rc::clone(node));
            }
            (Some(prev), Some(next)) => {
                node.borrow_mut().next = None;
                node.borrow_mut().prev = None;
                prev.borrow_mut().next = Some(Rc::clone(next));
                next.borrow_mut().prev = Some(Rc::clone(prev));

                let prev_tail = self.tail.as_ref().unwrap();
                prev_tail.borrow_mut().next = Some(Rc::clone(node));
                node.borrow_mut().prev = Some(Rc::clone(prev_tail));
                self.tail = Some(Rc::clone(node));
            }
        }
    }

    fn push_back(&mut self, node: &Rc<RefCell<Node<K, V>>>) {
        match &self.tail {
            None => {
                self.tail = Some(Rc::clone(node));
                self.head = Some(Rc::clone(node));
            }
            Some(prev_tail) => {
                prev_tail.borrow_mut().next = Some(Rc::clone(node));
                node.borrow_mut().prev = Some(Rc::clone(prev_tail));
                self.tail = Some(Rc::clone(node));
            }
        }
    }

    fn remove_head(&mut self) -> Option<Rc<RefCell<Node<K, V>>>> {
        if let Some(head) = self.head.as_ref().map(|a| a.clone()) {
            if let Some(next_head) = head.borrow_mut().next.as_ref() {
                next_head.borrow_mut().prev = None;
                self.head = Some(Rc::clone(next_head));
            } else {
                self.head = None;
                self.tail = None;
            }
            return Some(head);
        }
        None
    }
}

#[derive(Debug)]
pub struct LRUCache<K, V> {
    map: HashMap<K, Rc<RefCell<Node<K, V>>>>,
    cache_vals: LinkedList<K, V>,
    size: i32,
    capacity: i32,
}

impl<
        K: std::cmp::Eq + std::hash::Hash + Default + Clone + Display,
        V: Default + Clone + Display,
    > LRUCache<K, V>
{
    fn new(capacity: i32) -> Self {
        Self {
            map: HashMap::new(),
            cache_vals: LinkedList::<K, V>::new(),
            size: 0,
            capacity,
        }
    }

    fn get(&mut self, key: K) -> Option<V> {
        if let Some(node) = self.map.get(&key) {
            self.cache_vals.move_to_tail(node);
            return Some(node.borrow().value.clone());
        }
        None
    }

    fn set(&mut self, key: K, value: V) {
        if let Some(node) = self.map.get(&key) {
            self.cache_vals.move_to_tail(node);
            node.borrow_mut().value = value;
        } else {
            // remove the tail node if needed
            if self.size >= self.capacity {
                if let Some(prev_head) = self.cache_vals.remove_head() {
                    self.map.remove(&prev_head.borrow().key);
                };
            }
            // add node to list head
            let node = Rc::new(RefCell::new(Node::new(key.clone(), value)));
            self.cache_vals.push_back(&node);
            self.map.insert(key, node);
            self.size += 1;
        }
    }
    fn print(&mut self) {
        let mut head = self.cache_vals.head.as_ref().map(|a| a.clone());
        while !head.is_none() {
            let temp = head.map(|a| a.clone()).unwrap();
            print!("({}, {})", temp.borrow().key, temp.borrow().value);
            head = temp.borrow().next.clone();
        }
        println!("");
    }
}

fn main() {
    let mut obj = LRUCache::<i32, i32>::new(3);
    println!("The most recently watched title are: (key, value)");

    obj.set(10, 20);
    obj.print();

    obj.set(15, 25);
    obj.print();

    obj.set(20, 30);
    obj.print();

    obj.set(25, 35);
    obj.print();

    obj.set(5, 40);
    obj.print();

    obj.get(25);
    obj.print();
}

// Time complexity = O(1)
// Space complexity = o(k)
