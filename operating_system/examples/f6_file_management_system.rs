use std::cell::RefCell;
use std::rc::Rc;
#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct TrieNode {
    pub complete: bool,
    pub nodes: [Option<Box<TrieNode>>; 26],
}

#[derive(Default, Debug, PartialEq, Eq, Clone)]
pub struct FileSystem {
    pub canFind: bool,
    pub root: TrieNode,
}

impl FileSystem {
    fn new() -> Self {
        Default::default()
    }

    pub fn dfs(
        &mut self,
        r: Option<Box<TrieNode>>,
        w: String,
        names: &mut Vec<String>,
    ) -> Vec<String> {
        if r.is_none() {
            return names.to_vec();
        };
        if r.clone().unwrap().complete {
            names.push(w.to_string());
        }
        let mut j = 'a' as u8;
        while j <= 'z' as u8 {
            let n = format!("{}{:}", w, j as char);
            *names = self.dfs(
                r.clone().unwrap().nodes[(j - 'a' as u8) as usize].clone(),
                n.to_string(),
                &mut names.to_vec(),
            );
            j += 1;
        }
        return names.to_vec();
    }

    pub fn getAll(&mut self) -> Vec<String> {
        let mut names: Vec<String> = Vec::new();
        if Some(self.root.clone()).is_none() {
            return vec![];
        }

        return self.dfs(
            Some(Box::new(self.root.clone())),
            "".to_string(),
            &mut names,
        );
    }

    pub fn add(&mut self, word: String) {
        let n = word.len();
        let mut currNode = &mut self.root;
        let mut i = 0;
        for &c in word.as_bytes() {
            currNode =
                currNode.nodes[(c - b'a') as usize].get_or_insert(Box::new(Default::default()));
            if i == n - 1 {
                if currNode.complete == true {
                    println!("Word already present");
                    return;
                }
                currNode.complete = true;
            }
            i += 1;
        }
    }

    pub fn search(&mut self, word: String) -> bool {
        self.canFind = false;
        self.dfs2(Some(Box::new(self.root.clone())), word, 0);
        return self.canFind;
    }

    pub fn dfs2(&mut self, root: Option<Box<TrieNode>>, word: String, i: i32) {
        if self.canFind {
            return;
        };
        if root.is_none() {
            return;
        };
        let n = word.len();
        if n == i as usize {
            if root.unwrap().complete {
                self.canFind = true;
            }
            return;
        }
        if word.chars().nth(i as usize).unwrap() == '.' {
            let mut j = 'a' as u32;
            while j <= 'z' as u32 {
                self.dfs2(
                    root.clone().unwrap().nodes[(j - 'a' as u32) as usize].clone(),
                    word.to_string(),
                    i + 1,
                );
                j += 1;
            }
        } else {
            let index = word.chars().nth(i as usize).unwrap() as u32 - 'a' as u32;
            self.dfs2(
                root.clone().unwrap().nodes[index as usize].clone(),
                word.to_string(),
                i + 1,
            );
        }
    }
}

fn main() {
    println!("Initializing fsect");
    let mut fs = FileSystem::new();
    println!("Adding \'dir\' as a directory");
    fs.add("dir".to_string());
    println!("Adding \'dir\' as a directory again");
    fs.add("dir".to_string());
    println!("Adding \'dirr\' as a directory");
    fs.add("dirr".to_string());
    println!("Adding \'file\' as a file");
    fs.add("file".to_string());
    println!("Searching if \'.ile\' exists");
    println!("{:?}", fs.search(".ile".to_string()));
    println!("Searching if \'..ile\' exists");
    println!("{:?}", fs.search("..ile".to_string()));
    println!("Getting all of the files\\directories:");
    let result = fs.getAll();
    println!("{:?}", result);
}
