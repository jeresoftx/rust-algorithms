//! Trie and prefix-tree algorithms.
//!
//! # Example
//!
//! ```
//! use rust_algorithms::patterns::tries::Trie;
//!
//! let mut trie = Trie::new();
//! trie.insert("rust");
//! assert!(trie.starts_with("ru"));
//! assert!(trie.search("rust"));
//! ```

use std::collections::{BTreeMap, BTreeSet};

/// Returns the largest XOR obtainable from any pair of non-negative numbers.
///
/// Pattern: bit trie that prefers the opposite bit at each position.
/// Time: O(32n). Space: O(32n).
pub fn maximum_pair_xor(numbers: &[u32]) -> u32 {
    if numbers.len() < 2 {
        return 0;
    }

    let mut children = vec![[None, None]];
    for &number in numbers {
        let mut node = 0;
        for bit in (0..32).rev() {
            let branch = ((number >> bit) & 1) as usize;
            let next = match children[node][branch] {
                Some(next) => next,
                None => {
                    children.push([None, None]);
                    let next = children.len() - 1;
                    children[node][branch] = Some(next);
                    next
                }
            };
            node = next;
        }
    }

    numbers.iter().fold(0, |best, &number| {
        let mut node = 0;
        let mut candidate = 0;
        for bit in (0..32).rev() {
            let branch = ((number >> bit) & 1) as usize;
            let preferred = 1 - branch;
            let chooses_preferred = children[node][preferred].is_some();
            let chosen =
                children[node][preferred].unwrap_or_else(|| children[node][branch].unwrap());
            candidate |= u32::from(chooses_preferred) << bit;
            node = chosen;
        }
        best.max(candidate)
    })
}

/// Returns words that can be formed by concatenating at least two shorter words.
pub fn concatenated_words(words: &[&str]) -> Vec<String> {
    let mut ordered = words.to_vec();
    ordered.sort_unstable_by_key(|word| word.len());
    let mut dictionary = BTreeSet::new();
    let mut result = Vec::new();

    for word in ordered {
        if !word.is_empty() && can_concatenate(word, &dictionary) {
            result.push(word.to_string());
        }
        if !word.is_empty() {
            dictionary.insert(word);
        }
    }
    result.sort_unstable();
    result
}

fn can_concatenate(word: &str, dictionary: &BTreeSet<&str>) -> bool {
    let mut parts = vec![usize::MAX; word.len() + 1];
    parts[0] = 0;
    for start in 0..word.len() {
        if parts[start] == usize::MAX || !word.is_char_boundary(start) {
            continue;
        }
        for end in start + 1..=word.len() {
            if word.is_char_boundary(end) && dictionary.contains(&word[start..end]) {
                parts[end] = parts[end].min(parts[start] + 1);
            }
        }
    }
    parts[word.len()] >= 2 && parts[word.len()] != usize::MAX
}

#[derive(Default)]
struct TrieNode {
    children: BTreeMap<char, TrieNode>,
    is_word: bool,
}

/// Prefix tree for exact-word and prefix lookup.
pub struct Trie {
    root: TrieNode,
}

impl Trie {
    /// Creates an empty trie.
    pub fn new() -> Self {
        Self {
            root: TrieNode::default(),
        }
    }

    /// Inserts a word.
    pub fn insert(&mut self, word: &str) {
        let mut current = &mut self.root;

        for character in word.chars() {
            current = current.children.entry(character).or_default();
        }

        current.is_word = true;
    }

    /// Returns whether `word` was inserted exactly.
    pub fn search(&self, word: &str) -> bool {
        self.node_for(word).is_some_and(|node| node.is_word)
    }

    /// Returns whether any inserted word starts with `prefix`.
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

/// Word dictionary that supports `.` as a single-character wildcard.
pub struct WordDictionary {
    root: TrieNode,
}

impl WordDictionary {
    /// Creates an empty dictionary.
    pub fn new() -> Self {
        Self {
            root: TrieNode::default(),
        }
    }

    /// Adds one word.
    pub fn add_word(&mut self, word: &str) {
        let mut current = &mut self.root;

        for character in word.chars() {
            current = current.children.entry(character).or_default();
        }

        current.is_word = true;
    }

    /// Searches for an exact word or a pattern containing `.` wildcards.
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

/// Replaces words in a sentence with the shortest matching dictionary root.
///
/// Time: O(total characters)
/// Space: O(total dictionary characters)
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

/// Finds dictionary words that can be formed on a character board.
///
/// Pattern: trie-guided DFS.
/// Time: O(rows * cols * 4^word length) worst case
/// Space: O(total dictionary characters)
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
