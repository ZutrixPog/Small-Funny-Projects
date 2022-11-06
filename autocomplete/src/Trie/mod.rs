use std::collections::HashMap;

use crate::Autocomplete::Bank;

pub struct Trie {
    root: TrieNode
}

impl Trie {

    pub fn new() -> Self {
        Self {
            root: TrieNode::new()
        }
    }
    
    pub fn from_words(words: Vec<String>) -> Self {
        let mut trie = Self::new();
        for word in words {
            trie.insert(word);
        }
        
        trie
    }

    fn find_helper(root: Option<&mut TrieNode>, curr: &mut String, res: &mut Vec<String>, num: &mut i32) {
        if *num <= 0 {
            return;
        }

        if let Some(node) = root {
            if node.is_end() {
                *num -= 1;
                res.push(curr.clone());
            }

            for ch in 'a'..='z' {
                if let Some(next) = node.get(ch) {
                    curr.push(ch);
                    Self::find_helper(Some(next), curr, res, num);
                    curr.remove(curr.len()-1);
                }
            }
        }
    }
    
    pub fn contains(&mut self, word: String) -> bool {
        if let Some(node) = self.search_prefix(word) {
            node.is_end()
        } else {
            false
        }
    }
    
    pub fn starts_with(&mut self, prefix: String) -> bool {
        self.search_prefix(prefix).is_some()
    }

    fn search_prefix(&mut self, prefix: String) -> Option<&mut TrieNode> {
        let mut node = &mut self.root;
        let word = prefix.chars().collect::<Vec<char>>();
        
        for i in 0..word.len() {
            if node.contains_key(word[i]) {
                node = node.get(word[i]).unwrap();
            } else {
                return None;
            }
        }

        Some(node)
    }
}

impl Bank for Trie {
    fn find_words(&mut self, mut word: String, mut num: i32) -> Vec<String> {
        let mut res = Vec::new();

        if let Some(node) = self.search_prefix(word.clone()) {
            Self::find_helper(Some(node),&mut word, &mut res, &mut num);
        }

        res
    }

    fn insert(&mut self, word: String) {
        let mut node = &mut self.root;
        let word = word.chars().collect::<Vec<char>>();
        
        for i in 0..word.len() {
            node.links.entry(word[i]).or_insert(TrieNode::new());
            node = node.get(word[i]).unwrap();
        }
        node.set_end();
    }

}

struct TrieNode {
    links: HashMap<char, TrieNode>,
    is_end: bool
}

impl TrieNode {
    fn new() -> Self {
        Self {
            links: HashMap::new(),
            is_end: false
        }
    }
    
    fn contains_key(&self, c: char) -> bool {
        self.links.contains_key(&c)
    }
    
    fn get(&mut self, ch: char) -> Option<&mut TrieNode> {
        self.links.get_mut(&ch)
    }
    
    fn put(&mut self, ch: char, node: TrieNode) {
        self.links.insert(ch, node);
    }
    
    fn set_end(&mut self) {
        self.is_end = true;
    }
    
    fn is_end(&self) -> bool {
        self.is_end
    }
}