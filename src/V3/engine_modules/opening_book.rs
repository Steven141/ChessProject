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
        let openings: &str = include_str!("../opening_book.txt"); // provided path interpreted at compile time
        for opening in openings.lines() {
            book.trie.insert(opening);
        }
        book
    }
}
