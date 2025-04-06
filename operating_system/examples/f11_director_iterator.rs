// The following class is used for creating nested lists.
#[derive(Default, Debug, PartialEq, Eq, Clone)]
pub struct NestedDirectory {
    list: Vec<NestedDirectory>,
    file: Option<String>,
}

impl NestedDirectory {
    pub fn new(val: Option<String>) -> Self {
        if val.is_none() {
            Default::default()
        } else {
            NestedDirectory {
                file: val,
                list: vec![],
            }
        }
    }
    pub fn insert(&mut self, value: String) {
        self.file = Some(value);
    }
    pub fn isFile(self) -> bool {
        if !self.file.is_none() {
            return true;
        }
        return false;
    }
    pub fn getFile(self) -> String {
        return self.file.unwrap();
    }

    pub fn setFile(&mut self, value: String) {
        self.list.clear();
        self.file = Some(value.to_string());
    }
    pub fn add(&mut self, ni: NestedDirectory) {
        if !self.file.is_none() {
            self.list.push(NestedDirectory::new(self.file.clone()));
            self.file = None;
        }
        self.list.push(ni);
    }

    pub fn getList(self) -> Vec<NestedDirectory> {
        return self.list;
    }
}

#[derive(Default, Debug, PartialEq, Eq, Clone)]
pub struct NestedIterator {
    stack: Vec<NestedDirectory>,
}

impl NestedIterator {
    pub fn new(nestedList: Vec<NestedDirectory>) -> Self {
        let mut s: Vec<NestedDirectory> = Vec::new();
        let mut i = (nestedList.len() - 1) as i32;
        while i >= 0 {
            s.push(nestedList[i as usize].clone());
            i -= 1;
        }
        NestedIterator { stack: s }
    }

    pub fn hasNext(&mut self) -> bool {
        while !self.stack.is_empty() {
            let top = self.stack.last().unwrap();
            if top.clone().isFile() {
                return true;
            }
            let topList: Vec<NestedDirectory> = self.stack.pop().unwrap().getList().to_vec();
            let mut i = (topList.len() - 1) as i32;
            while i >= 0 {
                self.stack.push(topList[i as usize].clone());
                i -= 1;
            }
        }
        return false;
    }

    pub fn next(&mut self) -> String {
        if self.hasNext() {
            let top = self.stack.pop().unwrap();
            return top.clone().getFile();
        }
        return "".to_string();
    }
}

fn main() {
    let mut nestedList: Vec<NestedDirectory> = Vec::new();
    let mut l1: NestedDirectory = NestedDirectory::new(None);
    nestedList.push(NestedDirectory::new(Some("F1".to_string())));
    l1.add(NestedDirectory::new(Some("F2".to_string())));
    l1.add(NestedDirectory::new(Some("D1".to_string())));
    nestedList.push(l1);
    nestedList.push(NestedDirectory::new(Some("D2".to_string())));
    let mut itr: NestedIterator = NestedIterator::new(nestedList);
    println!("Original structure: [F1, [F2, D1], D2]");
    println!("Output:");
    while itr.hasNext() {
        print!("itr.next(): ");
        println!("{}", itr.next());
    }
}

// Time complexity = O(n+l)
// Space complexity = O(n+l)
