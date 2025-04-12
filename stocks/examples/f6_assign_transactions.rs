#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
// helper function for test
pub fn reverse_kgroup(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
    let mut head = head;
    let mut node = &mut head;
    for _ in 0..k {
        if let Some(n) = node {
            node = &mut n.next;
        } else {
            return head;
        }
    }
    let mut newHead = reverse_k_group(node.take(), k);
    while let Some(h) = head.take() {
        newHead = Some(Box::new(ListNode {
            val: h.val,
            next: newHead,
        }));
        head = h.next;
    }
    newHead
}

pub fn to_list(vec: Vec<i32>) -> Option<Box<ListNode>> {
    let mut current = None;
    for &v in vec.iter().rev() {
        let mut node = ListNode::new(v);
        node.next = current;
        current = Some(Box::new(node));
    }
    current
}

pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
    let mut dummy_head = Some(Box::new(ListNode { val: 0, next: head }));
    let mut head = dummy_head.as_mut();
    'outer: loop {
        let mut start = head.as_mut().unwrap().next.take();
        if start.is_none() {
            break 'outer;
        }
        let mut end = start.as_mut();
        for _ in 0..(k - 1) {
            end = end.unwrap().next.as_mut();
            if end.is_none() {
                head.as_mut().unwrap().next = start;
                break 'outer;
            }
        }
        let mut tail = end.as_mut().unwrap().next.take();
        // BEFORE: head -> start -> 123456... -> end   -> tail
        // AFTER:  head -> end   -> ...654321 -> start -> tail
        let end = reverse(start, tail);
        head.as_mut().unwrap().next = end;
        for _ in 0..k {
            head = head.unwrap().next.as_mut()
        }
    }
    dummy_head.unwrap().next
}

#[inline(always)]
fn reverse(mut head: Option<Box<ListNode>>, tail: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut prev = tail;
    let mut current = head;
    while let Some(mut current_node_inner) = current {
        let mut next = current_node_inner.next.take();
        current_node_inner.next = prev.take();
        prev = Some(current_node_inner);
        current = next;
    }
    prev
}

fn main() {
    let mut v = to_list(vec![1, 2, 3, 4, 5]);
    let l = v.clone();
    print(&mut v);
    print(&mut reverse_kgroup(l, 2));
}

// Time complexity = O(n)
// Space complexity = O(1)
