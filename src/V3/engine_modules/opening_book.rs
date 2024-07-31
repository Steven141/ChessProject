//! Module holding information about the opening book.


use pyo3::prelude::*;
use std::collections::HashMap;


#[pyclass(module = "ChessProject", get_all, set_all)]
#[derive(Clone, Debug, Default)]
struct TrieNode {
    children: HashMap<String, TrieNode>,
    terminal: bool,
}


#[pyclass(module = "ChessProject", get_all, set_all)]
#[derive(Clone, Debug, Default)]
struct Trie {
    root: TrieNode,
}


#[pymethods]
impl Trie {
    #[new]
    pub fn new() -> Self {
        Trie {
            root: TrieNode::default(),
        }
    }


    pub fn insert(&mut self, moves: &str) {
        let mut curr_node: &mut TrieNode = &mut self.root;
        for i in (0..moves.len()).step_by(4) {
            curr_node = curr_node.children.entry(moves[i..i+4].to_string()).or_default();
        }
        curr_node.terminal = true;
    }


    pub fn contains(&self, moves: &str) -> bool {
        let mut curr_node: &TrieNode = &self.root;
        for i in (0..moves.len()).step_by(4) {
            match curr_node.children.get(&moves[i..i+4]) {
                Some(node) => curr_node = node,
                None => return false,
            }
        }
        curr_node.terminal
    }
}


#[pyclass(module = "ChessProject", get_all, set_all)]
#[derive(Clone, Debug)]
pub struct OpeningBook {
    trie: Trie,
}


#[pymethods]
impl OpeningBook {
    #[new]
    pub fn new() -> Self {
        let mut book: OpeningBook = OpeningBook {
            trie: Trie::new(),
        };
        let openings: &str = include_str!("../opening_book.txt");
        for opening in openings.lines() {
            book.trie.insert(opening);
        }
        book
    }
}


/// Tests


#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn trie_test() {
        // let mut t: Trie = Trie::new();
        // t.insert("g1f3g8f6c2c4");
        // t.insert("g1f3g8f6d2d4");
        // let mut tn: &TrieNode = &t.root;
        // println!("{:?}", tn.children.contains_key("g1f3"));
        // tn = tn.children.get("g1f3").unwrap();
        // println!("{:?}", tn);
        // tn = tn.children.get("g8f6").unwrap();
        // println!("{:?}, {}", tn, tn.terminal);
        // tn = tn.children.get("c2c4").unwrap();
        // println!("{:?}, {}", tn, tn.terminal);
        let ob: OpeningBook = OpeningBook::new();
        println!("{}", ob.trie.root.children.contains_key("g1f3"));
        println!("{}", algebra_to_move!("c2c4"));
        panic!();
    }
}
