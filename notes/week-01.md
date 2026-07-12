# Week 01: Rust Basics, Hashing, Arrays, and Strings

## Goal

Build enough Rust fluency to solve common interview warmups without getting blocked by syntax, ownership, or standard collections.

## Problems Completed

| Problem | Pattern | Function | Tests |
| --- | --- | --- | --- |
| Two Sum | Hash map complement lookup | `two_sum` | 3 |
| Valid Anagram | Frequency counting | `valid_anagram` | 3 |
| Contains Duplicate | Set membership | `contains_duplicate` | 2 |
| Group Anagrams | Canonical hash key | `group_anagrams` | 2 |
| Product of Array Except Self | Prefix/suffix products | `product_except_self` | 3 |
| Top K Frequent Elements | Frequency counting + sorting | `top_k_frequent` | 2 |
| Longest Consecutive Sequence | Set sequence starts | `longest_consecutive` | 3 |

## Rust Concepts Practiced

- `Vec<T>` ownership and iteration.
- `HashMap` for counts, indexes, and grouping.
- `HashSet` for membership checks.
- `Option` for functions that may not find an answer.
- Integration tests in `tests/hashing_test.rs`.
- `cargo fmt` and `cargo test` as the default verification loop.

## Interview Patterns

### Complement Lookup

Use when a problem asks for two values that combine into a target.

Invariant:

- Before processing index `i`, the map contains values from indexes `< i`.
- If `target - nums[i]` exists in the map, the answer is found.

### Frequency Counting

Use when comparing character counts, finding common values, or ranking by occurrence.

Questions to ask:

- Do I need exact counts or just presence?
- Is sorting simpler than hashing?
- Does the output order matter?

### Canonical Keys

Use when values can be normalized into the same representation.

Example:

- `"eat"`, `"tea"`, and `"ate"` all normalize to `"aet"`.

### Prefix/Suffix

Use when each index needs information from the left and the right.

Invariant:

- First pass stores the product before each index.
- Reverse pass multiplies by the product after each index.

### Sequence Starts

Use when finding consecutive runs in an unordered collection.

Invariant:

- Only start counting at `x` when `x - 1` is absent.
- This prevents recounting the same sequence.

## Commands

```bash
cargo fmt
cargo test
```

## Retrospective

- The strongest pattern this week is direct hash lookup.
- The most important habit is writing the test before the implementation.
- The next week should introduce two pointers and sliding windows, because those complement hashing well in Google-style interviews.
