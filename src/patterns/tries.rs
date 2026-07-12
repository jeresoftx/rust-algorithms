use std::collections::BTreeMap;

#[derive(Default)]
struct TrieNode {
    children: BTreeMap<char, TrieNode>,
    is_word: bool,
}

pub struct Trie {
    root: TrieNode,
}

impl Trie {
    pub fn new() -> Self {
        Self {
            root: TrieNode::default(),
        }
    }

    pub fn insert(&mut self, word: &str) {
        let mut current = &mut self.root;

        for character in word.chars() {
            current = current.children.entry(character).or_default();
        }

        current.is_word = true;
    }

    pub fn search(&self, word: &str) -> bool {
        self.node_for(word).is_some_and(|node| node.is_word)
    }

    pub fn starts_with(&self, prefix: &str) -> bool {
        self.node_for(prefix).is_some()
    }

    fn shortest_root(&self, word: &str) -> Option<String> {
        let mut current = &self.root;
        let mut prefix = String::new();

        for character in word.chars() {
            current = current.children.get(&character)?;
            prefix.push(character);

            if current.is_word {
                return Some(prefix);
            }
        }

        None
    }

    fn node_for(&self, text: &str) -> Option<&TrieNode> {
        let mut current = &self.root;

        for character in text.chars() {
            current = current.children.get(&character)?;
        }

        Some(current)
    }
}

impl Default for Trie {
    fn default() -> Self {
        Self::new()
    }
}

pub struct WordDictionary {
    root: TrieNode,
}

impl WordDictionary {
    pub fn new() -> Self {
        Self {
            root: TrieNode::default(),
        }
    }

    pub fn add_word(&mut self, word: &str) {
        let mut current = &mut self.root;

        for character in word.chars() {
            current = current.children.entry(character).or_default();
        }

        current.is_word = true;
    }

    pub fn search(&self, pattern: &str) -> bool {
        let characters: Vec<char> = pattern.chars().collect();
        wildcard_search(&self.root, &characters, 0)
    }
}

impl Default for WordDictionary {
    fn default() -> Self {
        Self::new()
    }
}

pub fn replace_words(dictionary: Vec<&str>, sentence: &str) -> String {
    let mut trie = Trie::new();

    for root in dictionary {
        trie.insert(root);
    }

    sentence
        .split_whitespace()
        .map(|word| trie.shortest_root(word).unwrap_or_else(|| word.to_string()))
        .collect::<Vec<_>>()
        .join(" ")
}

fn wildcard_search(node: &TrieNode, pattern: &[char], index: usize) -> bool {
    if index == pattern.len() {
        return node.is_word;
    }

    let character = pattern[index];

    if character == '.' {
        return node
            .children
            .values()
            .any(|child| wildcard_search(child, pattern, index + 1));
    }

    node.children
        .get(&character)
        .is_some_and(|child| wildcard_search(child, pattern, index + 1))
}
