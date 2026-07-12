use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

pub type TreeLink = Option<Rc<RefCell<TreeNode>>>;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: TreeLink,
    pub right: TreeLink,
}

impl TreeNode {
    pub fn new(val: i32) -> Self {
        Self {
            val,
            left: None,
            right: None,
        }
    }
}

/// Build a binary tree from level-order values.
///
/// Time: O(n)
/// Space: O(n)
pub fn tree_from_level_order(values: Vec<Option<i32>>) -> TreeLink {
    let Some(Some(root_value)) = values.first() else {
        return None;
    };

    let root = Rc::new(RefCell::new(TreeNode::new(*root_value)));
    let mut queue = VecDeque::from([Rc::clone(&root)]);
    let mut index = 1;

    while index < values.len() {
        let Some(parent) = queue.pop_front() else {
            break;
        };

        if let Some(Some(value)) = values.get(index) {
            let child = Rc::new(RefCell::new(TreeNode::new(*value)));
            parent.borrow_mut().left = Some(Rc::clone(&child));
            queue.push_back(child);
        }
        index += 1;

        if let Some(Some(value)) = values.get(index) {
            let child = Rc::new(RefCell::new(TreeNode::new(*value)));
            parent.borrow_mut().right = Some(Rc::clone(&child));
            queue.push_back(child);
        }
        index += 1;
    }

    Some(root)
}

/// Convert a binary tree to compact level-order values.
///
/// Time: O(n)
/// Space: O(n)
pub fn tree_to_level_order(root: TreeLink) -> Vec<Option<i32>> {
    let mut result = Vec::new();
    let mut queue = VecDeque::from([root]);

    while let Some(node) = queue.pop_front() {
        if let Some(node) = node {
            let node = node.borrow();
            result.push(Some(node.val));
            queue.push_back(node.left.clone());
            queue.push_back(node.right.clone());
        } else {
            result.push(None);
        }
    }

    while result.last() == Some(&None) {
        result.pop();
    }

    result
}

/// Maximum Depth of Binary Tree
///
/// Pattern: DFS with return value.
/// Idea: depth is one plus the maximum depth of either child.
///
/// Time: O(n)
/// Space: O(h)
pub fn max_depth(root: TreeLink) -> i32 {
    match root {
        None => 0,
        Some(node) => {
            let node = node.borrow();
            1 + max_depth(node.left.clone()).max(max_depth(node.right.clone()))
        }
    }
}

/// Invert Binary Tree
///
/// Pattern: DFS mutation.
/// Idea: recursively invert children, then swap them.
///
/// Time: O(n)
/// Space: O(h)
pub fn invert_tree(root: TreeLink) -> TreeLink {
    if let Some(node) = root.clone() {
        let mut node = node.borrow_mut();
        let left = invert_tree(node.left.take());
        let right = invert_tree(node.right.take());
        node.left = right;
        node.right = left;
    }

    root
}

/// Diameter of Binary Tree
///
/// Pattern: DFS returning height while tracking best path.
/// Idea: each node offers a path through left height + right height.
///
/// Time: O(n)
/// Space: O(h)
pub fn diameter_of_binary_tree(root: TreeLink) -> i32 {
    let mut best = 0;
    height_and_diameter(root, &mut best);
    best
}

fn height_and_diameter(root: TreeLink, best: &mut i32) -> i32 {
    match root {
        None => 0,
        Some(node) => {
            let node = node.borrow();
            let left = height_and_diameter(node.left.clone(), best);
            let right = height_and_diameter(node.right.clone(), best);
            *best = (*best).max(left + right);
            1 + left.max(right)
        }
    }
}

/// Balanced Binary Tree
///
/// Pattern: DFS with sentinel.
/// Idea: return -1 when a subtree is already unbalanced.
///
/// Time: O(n)
/// Space: O(h)
pub fn is_balanced(root: TreeLink) -> bool {
    balanced_height(root) != -1
}

fn balanced_height(root: TreeLink) -> i32 {
    match root {
        None => 0,
        Some(node) => {
            let node = node.borrow();
            let left = balanced_height(node.left.clone());
            if left == -1 {
                return -1;
            }

            let right = balanced_height(node.right.clone());
            if right == -1 || (left - right).abs() > 1 {
                return -1;
            }

            1 + left.max(right)
        }
    }
}

/// Same Tree
///
/// Pattern: paired DFS.
/// Idea: both trees match if roots, left subtrees, and right subtrees match.
///
/// Time: O(n)
/// Space: O(h)
pub fn is_same_tree(left: TreeLink, right: TreeLink) -> bool {
    match (left, right) {
        (None, None) => true,
        (Some(left), Some(right)) => {
            let left = left.borrow();
            let right = right.borrow();

            left.val == right.val
                && is_same_tree(left.left.clone(), right.left.clone())
                && is_same_tree(left.right.clone(), right.right.clone())
        }
        _ => false,
    }
}

/// Subtree of Another Tree
///
/// Pattern: DFS plus same-tree comparison.
/// Idea: each root is a candidate match for the subtree.
///
/// Time: O(n * m)
/// Space: O(h)
pub fn is_subtree(root: TreeLink, sub_root: TreeLink) -> bool {
    if sub_root.is_none() {
        return true;
    }

    let Some(root_node) = root else {
        return false;
    };

    if is_same_tree(Some(Rc::clone(&root_node)), sub_root.clone()) {
        return true;
    }

    let root_node = root_node.borrow();
    is_subtree(root_node.left.clone(), sub_root.clone())
        || is_subtree(root_node.right.clone(), sub_root)
}

/// Binary Tree Level Order Traversal
///
/// Pattern: BFS by levels.
/// Idea: process exactly the current queue length for each level.
///
/// Time: O(n)
/// Space: O(n)
pub fn level_order(root: TreeLink) -> Vec<Vec<i32>> {
    let Some(root) = root else {
        return Vec::new();
    };

    let mut result = Vec::new();
    let mut queue = VecDeque::from([root]);

    while !queue.is_empty() {
        let level_len = queue.len();
        let mut level = Vec::with_capacity(level_len);

        for _ in 0..level_len {
            let node = queue.pop_front().expect("queue length was captured");
            let node = node.borrow();
            level.push(node.val);

            if let Some(left) = node.left.clone() {
                queue.push_back(left);
            }

            if let Some(right) = node.right.clone() {
                queue.push_back(right);
            }
        }

        result.push(level);
    }

    result
}
