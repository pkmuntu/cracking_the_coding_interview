use std::collections::LinkedList;
fn idle_time(list1: LinkedList<u32>, list2: LinkedList<u32>) -> LinkedList<u32> {
    let mut dummy: LinkedList<u32> = LinkedList::new();
    dummy.push_back(0);
    let p: LinkedList<u32> = list1;
    let q: LinkedList<u32> = list2;
    let mut curr: LinkedList<u32> = dummy.clone();
    let mut carry = 0;
    let mut piter = p.iter();
    let mut qiter = q.iter();
    let mut i = 0;
    while i < p.len() {
        let x = piter.next().unwrap();
        let y = qiter.next().unwrap();
        let sum = carry + x + y;
        carry = sum / 10;
        curr.push_back(sum % 10);
        i += 1;
    }
    if carry > 0 {
        curr.push_back(carry);
    }
    curr.pop_front();
    return curr;
}

fn main() {
    // driver code
    let mut list: LinkedList<u32> = LinkedList::new();

    list.push_back(9);
    list.push_back(9);
    list.push_back(9);

    let mut list2: LinkedList<u32> = LinkedList::new();
    list2.push_back(9);
    list2.push_back(9);
    list2.push_back(9);
    let dummy = idle_time(list, list2);
    println!("{:?}", dummy);
}

// Time complexity = O(max(m,n))
// Space complexity = o(max(m,n))
