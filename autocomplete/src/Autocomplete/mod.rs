use crate::Trie::Trie;

pub trait Bank {
    fn insert(&mut self, word: String);
    fn find_words(&mut self, prefix: String, num: i32) -> Vec<String>;
}

pub struct Autocompleter<B: Bank> {
    bank: B,
    limit: i32
}

impl<B: Bank> Autocompleter<B> {
    pub fn new(bank: B) -> Self {
        Self {
            bank: bank,
            limit: 5
        }
    }

    pub fn load_words(&mut self, words: Vec<String>) {
        for word in words {
            self.bank.insert(word);
        }
    }

    pub fn set_limit(&mut self, limit: i32) {
        self.limit = limit;
    }

    pub fn auto_complete(&mut self, prefix: String) -> Vec<String> {
        self.bank.find_words(prefix, self.limit)
    }
}

impl Autocompleter<Trie> {
    pub fn new_with_trie() -> Self {
        Self {
            bank: Trie::new(),
            limit: 5
        }
    }
}