use std::cell::RefCell;
use std::rc::Rc;

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct TrieNode {
    pub is_word: bool,
    pub children: [Option<Box<TrieNode>>; 26],
}

#[derive(Default, Debug, PartialEq, Eq, Clone)]
pub struct Trie {
    pub root: TrieNode,
}

impl Trie {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn add(&mut self, key: String) {
        let n = key.len();
        let mut node = &mut self.root;
        let mut index = 0;

        for c in key.as_bytes() {
            node = node.children[(c - b'A') as usize].get_or_insert(Box::new(Default::default()));
        }

        node.is_word = true;
    }

    pub fn search(&mut self, key: String) -> bool {
        let mut node = &mut self.root;

        for c in key.as_bytes() {
            if node.children[(c - b'A') as usize].is_none() {
                return false;
            } else {
                node =
                    node.children[(c - b'A') as usize].get_or_insert(Box::new(Default::default()));
            }
        }

        if node.is_word {
            return true;
        }
        return false;
    }
}

fn searchWords(mut grid: &mut Vec<Vec<char>>, words: Vec<String>, mut result: &mut Vec<String>) {
    let mut t = Trie::new();

    // Inserting words in dictionary
    for word in words.into_iter() {
        t.add(word.to_string());
    }

    for row in 0..5 {
        for col in 0..5 {
            dfs(
                &mut t.root,
                &mut grid,
                row as i32,
                col as i32,
                &mut result,
                &mut "".to_string(),
            );
        }
    }
}

fn dfs(
    node: &mut TrieNode,
    mut grid: &mut Vec<Vec<char>>,
    row: i32,
    col: i32,
    mut result: &mut Vec<String>,
    word: &String,
) {
    // checking if we found the word
    if node.is_word {
        result.push(word.to_string());
        node.is_word = false;
    }

    let mut some_word = word.to_string();
    if 0 <= row && row < 5 && 0 <= col && col < 5 {
        let c = grid[row as usize][col as usize];
        let mut index: i32 = -1;
        if c as u8 >= b'A' {
            index = (c as u8 - b'A') as i32;
        }

        if index != -1 {
            if c != '#' && !node.children[index as usize].is_none() {
                let child =
                    node.children[index as usize].get_or_insert(Box::new(Default::default()));
                some_word.push(c);
                grid[row as usize][col as usize] = '#';

                let offsets: Vec<Vec<i32>> = vec![vec![0, 1], vec![1, 0], vec![0, -1], vec![-1, 0]];
                for offset in offsets.into_iter() {
                    let rowOffset = offset[0];
                    let colOffset = offset[1];
                    dfs(
                        child,
                        &mut grid,
                        row + rowOffset,
                        col + colOffset,
                        &mut result,
                        &some_word,
                    );
                }
                // resotring state after exploration
                grid[row as usize][col as usize] = c;
            }
        }
    }
}
fn main() {
    let mut grid: Vec<Vec<char>> = vec![
        vec!['B', 'S', 'L', 'I', 'M'],
        vec!['R', 'I', 'L', 'M', 'O'],
        vec!['O', 'L', 'I', 'E', 'O'],
        vec!['R', 'Y', 'I', 'L', 'N'],
        vec!['B', 'U', 'N', 'E', 'C'],
    ];

    let words: Vec<String> = vec!["BUY", "SLICK", "SLIME", "ONLINE", "NOW"]
        .into_iter()
        .map(String::from)
        .collect();

    let mut result: Vec<String> = Vec::new();
    searchWords(&mut grid, words, &mut result);
    println!("{:?}", result);
}

// Time complexity = O(n * n^l)
// Space complexity = O(m)
