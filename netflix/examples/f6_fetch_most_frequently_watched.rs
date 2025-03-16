use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug, Clone, Default)]
struct Node<K, V>
where
    K: Default,
    V: Default,
{
    key: K,
    value: V,
    freq: i32,
    prev: Option<Rc<RefCell<Node<K, V>>>>,
    next: Option<Rc<RefCell<Node<K, V>>>>,
}

impl<K, V> Node<K, V>
where
    K: Default,
    V: Default,
{
    pub fn new(key: K, value: V) -> Self {
        Self {
            key: key,
            value: value,
            freq: 1,
            prev: None,
            next: None,
        }
    }
}

#[derive(Debug, Default)]
struct LinkedList<K, V>
where
    K: Default,
    V: Default,
{
    head: Option<Rc<RefCell<Node<K, V>>>>,
    tail: Option<Rc<RefCell<Node<K, V>>>>,
}

impl<K, V> LinkedList<K, V>
where
    K: Default,
    V: Default,
{
    fn push_back(&mut self, node: &Rc<RefCell<Node<K, V>>>) {
        match &self.tail {
            None => {
                self.head = Some(Rc::clone(node));
                self.tail = Some(Rc::clone(node));
            }
            Some(prev_tail) => {
                prev_tail.borrow_mut().next = Some(Rc::clone(node));
                node.borrow_mut().prev = Some(Rc::clone(prev_tail));
                self.tail = Some(Rc::clone(node));
            }
        }
    }

    fn pop_front(&mut self) -> Option<Rc<RefCell<Node<K, V>>>> {
        if let Some(head) = self.head.as_ref().map(|a| a.clone()) {
            if let Some(head_next) = head.borrow_mut().next.take() {
                head_next.borrow_mut().prev = None;
                self.head = Some(head_next);
            } else {
                self.tail = None;
                self.head = None;
            }
            return Some(head);
        }
        None
    }

    fn is_empty(&self) -> bool {
        self.head.is_none() && self.tail.is_none()
    }
}

struct LfuStructure<K, V>
where
    K: Eq + std::hash::Hash + Copy + Default + std::fmt::Display,
    V: Clone + Default + std::fmt::Display,
{
    capacity: i32,
    count: i32,
    min_freq: RefCell<i32>,
    key_dict: HashMap<K, Rc<RefCell<Node<K, V>>>>,
    freq_dict: RefCell<HashMap<i32, LinkedList<K, V>>>,
}

impl<K, V> LfuStructure<K, V>
where
    K: Eq + std::hash::Hash + Copy + Default + std::fmt::Display,
    V: Clone + Default + std::fmt::Display,
{
    pub fn new(capacity: i32) -> Self {
        Self {
            capacity,
            count: 0,
            min_freq: RefCell::new(0),
            key_dict: HashMap::new(),
            freq_dict: RefCell::new(HashMap::new()),
        }
    }

    fn min_freq(&self) -> i32 {
        *self.min_freq.borrow()
    }

    fn set_min_freq(&self, freq: i32) {
        *self.min_freq.borrow_mut() = freq;
    }

    fn get(&self, key: K) -> Option<V> {
        if self.capacity == 0 {
            return None;
        }
        if let Some(node) = self.key_dict.get(&key) {
            let value = node.borrow().value.clone();
            self.update_freq(node.clone());
            return Some(value);
        } else {
            return None;
        }
    }

    fn set(&mut self, key: K, val: V) {
        if self.capacity == 0 {
            return;
        }

        if let Some(node) = self.key_dict.get(&key) {
            node.borrow_mut().value = val;
            self.update_freq(node.clone());
        } else {
            if self.count >= self.capacity {
                if let Some(node) = self.pop_front_node(self.min_freq()) {
                    self.key_dict.remove(&node.borrow().key);
                    self.count -= 1;
                }
            }

            let node = Rc::new(RefCell::new(Node::new(key.clone(), val)));
            self.key_dict.insert(key.clone(), node.clone());

            self.freq_dict
                .borrow_mut()
                .entry(1)
                .or_default()
                .push_back(&node);

            self.set_min_freq(1);
            self.count += 1;
        }
    }

    fn update_freq(&self, node: Rc<RefCell<Node<K, V>>>) {
        let freq = node.borrow().freq;
        node.borrow_mut().freq += 1;
        self.push_back_node(freq + 1, self.take_node(freq, node));
        if freq == self.min_freq()
            && self
                .freq_dict
                .borrow_mut()
                .entry(freq)
                .or_default()
                .is_empty()
        {
            self.set_min_freq(freq + 1);
        }
    }

    fn take_node(&self, freq: i32, node: Rc<RefCell<Node<K, V>>>) -> Rc<RefCell<Node<K, V>>> {
        let mut freq_dict = self.freq_dict.borrow_mut();
        let linked_list = freq_dict.get_mut(&freq).unwrap();
        {
            let mut node = node.borrow_mut();
            match (node.prev.take(), node.next.take()) {
                (Some(prev), Some(next)) => {
                    next.borrow_mut().prev = Some(prev.clone());
                    prev.borrow_mut().next = Some(next);
                }
                (None, Some(next)) => {
                    next.borrow_mut().prev = None;
                    linked_list.head = Some(next);
                }
                (Some(prev), None) => {
                    prev.borrow_mut().next = None;
                    linked_list.tail = Some(prev);
                }
                (None, None) => {
                    linked_list.head = None;
                    linked_list.tail = None;
                }
            }
        }
        node
    }

    fn push_back_node(&self, freq: i32, node: Rc<RefCell<Node<K, V>>>) {
        let mut freq_dict = self.freq_dict.borrow_mut();
        let linked_list = freq_dict.entry(freq).or_default();
        linked_list.push_back(&node);
    }

    fn pop_front_node(&self, freq: i32) -> Option<Rc<RefCell<Node<K, V>>>> {
        if let Some(linked_list) = self.freq_dict.borrow_mut().get_mut(&freq) {
            linked_list.pop_front()
        } else {
            None
        }
    }

    fn print(&self) {
        for (key, value) in self.key_dict.iter() {
            print!("({}, {})", key, value.borrow().value);
        }
        println!("");
    }
}

fn main() {
    let mut obj = LfuStructure::<i32, i32>::new(2);
    println!("The most frequently watched titles are: (key, value)");

    obj.set(1, 1);
    obj.set(2, 2);
    obj.print();

    obj.get(1);

    obj.set(3, 3);
    obj.print();

    obj.get(2);
    obj.set(4, 4);
    obj.print();

    obj.get(1);
    obj.get(3);
    obj.get(4);
    obj.print();
}
