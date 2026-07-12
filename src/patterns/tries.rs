use std::collections::{BTreeMap, BTreeSet};

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

pub fn find_words(mut board: Vec<Vec<char>>, words: Vec<&str>) -> Vec<String> {
    if board.is_empty() || board[0].is_empty() || words.is_empty() {
        return Vec::new();
    }

    let mut trie = Trie::new();
    for word in words {
        if !word.is_empty() {
            trie.insert(word);
        }
    }

    let rows = board.len();
    let cols = board[0].len();
    let mut found = BTreeSet::new();
    let mut current = String::new();

    for row in 0..rows {
        for col in 0..cols {
            collect_board_words(row, col, &mut board, &trie.root, &mut current, &mut found);
        }
    }

    found.into_iter().collect()
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

fn collect_board_words(
    row: usize,
    col: usize,
    board: &mut [Vec<char>],
    node: &TrieNode,
    current: &mut String,
    found: &mut BTreeSet<String>,
) {
    if board[row][col] == '\0' {
        return;
    }

    let character = board[row][col];
    let Some(next_node) = node.children.get(&character) else {
        return;
    };

    current.push(character);

    if next_node.is_word {
        found.insert(current.clone());
    }

    board[row][col] = '\0';

    for (next_row, next_col) in board_neighbors(row, col, board.len(), board[0].len()) {
        collect_board_words(next_row, next_col, board, next_node, current, found);
    }

    board[row][col] = character;
    current.pop();
}

fn board_neighbors(row: usize, col: usize, rows: usize, cols: usize) -> Vec<(usize, usize)> {
    let mut result = Vec::with_capacity(4);
    let directions = [(1, 0), (-1, 0), (0, 1), (0, -1)];

    for (row_delta, col_delta) in directions {
        let next_row = row as isize + row_delta;
        let next_col = col as isize + col_delta;

        if next_row >= 0 && next_row < rows as isize && next_col >= 0 && next_col < cols as isize {
            result.push((next_row as usize, next_col as usize));
        }
    }

    result
}
