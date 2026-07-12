use rust_algorithms::patterns::tries::{find_words, replace_words, Trie, WordDictionary};

#[test]
fn trie_searches_inserted_words_exactly() {
    let mut trie = Trie::new();

    trie.insert("apple");

    assert!(trie.search("apple"));
    assert!(!trie.search("app"));
}

#[test]
fn trie_checks_prefixes_without_requiring_full_word() {
    let mut trie = Trie::new();

    trie.insert("apple");

    assert!(trie.starts_with("app"));
    assert!(!trie.starts_with("apl"));
}

#[test]
fn trie_distinguishes_prefix_word_after_explicit_insert() {
    let mut trie = Trie::new();

    trie.insert("apple");
    trie.insert("app");

    assert!(trie.search("app"));
    assert!(trie.search("apple"));
}

#[test]
fn word_dictionary_matches_exact_words() {
    let mut dictionary = WordDictionary::new();

    dictionary.add_word("bad");
    dictionary.add_word("dad");
    dictionary.add_word("mad");

    assert!(dictionary.search("bad"));
    assert!(!dictionary.search("pad"));
}

#[test]
fn word_dictionary_matches_single_character_wildcards() {
    let mut dictionary = WordDictionary::new();

    dictionary.add_word("bad");
    dictionary.add_word("dad");
    dictionary.add_word("mad");

    assert!(dictionary.search(".ad"));
    assert!(dictionary.search("b.."));
    assert!(!dictionary.search(".."));
}

#[test]
fn word_dictionary_handles_multiple_word_lengths() {
    let mut dictionary = WordDictionary::new();

    dictionary.add_word("at");
    dictionary.add_word("atom");

    assert!(dictionary.search("a."));
    assert!(dictionary.search("a..."));
    assert!(!dictionary.search("a.."));
}

#[test]
fn replace_words_uses_shortest_dictionary_root() {
    let dictionary = vec!["cat", "bat", "rat"];
    let sentence = "the cattle was rattled by the battery";

    assert_eq!(
        replace_words(dictionary, sentence),
        "the cat was rat by the bat"
    );
}

#[test]
fn replace_words_prefers_shorter_root_when_many_match() {
    let dictionary = vec!["a", "aa", "aaa", "aaaa"];
    let sentence = "a aa a aaaa aaa aaa aaa aaaaaa bbb baba ababa";

    assert_eq!(
        replace_words(dictionary, sentence),
        "a a a a a a a a bbb baba a"
    );
}

#[test]
fn replace_words_keeps_words_without_matching_root() {
    let dictionary = vec!["blue", "green"];
    let sentence = "red yellow bluebird";

    assert_eq!(replace_words(dictionary, sentence), "red yellow blue");
}

#[test]
fn find_words_returns_dictionary_words_present_on_board() {
    let board = vec![
        vec!['o', 'a', 'a', 'n'],
        vec!['e', 't', 'a', 'e'],
        vec!['i', 'h', 'k', 'r'],
        vec!['i', 'f', 'l', 'v'],
    ];
    let words = vec!["oath", "pea", "eat", "rain"];

    assert_eq!(find_words(board, words), vec!["eat", "oath"]);
}

#[test]
fn find_words_does_not_reuse_the_same_cell_in_one_word() {
    let board = vec![vec!['a', 'b']];
    let words = vec!["aba", "ab"];

    assert_eq!(find_words(board, words), vec!["ab"]);
}

#[test]
fn find_words_handles_empty_inputs() {
    assert!(find_words(Vec::new(), vec!["a"]).is_empty());
    assert!(find_words(vec![vec!['a']], Vec::new()).is_empty());
}
