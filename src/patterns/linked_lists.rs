#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    pub fn new(val: i32) -> Self {
        Self { val, next: None }
    }
}

pub type Link = Option<Box<ListNode>>;

/// Build a linked list from values.
///
/// Time: O(n)
/// Space: O(n)
pub fn list_from_vec(values: Vec<i32>) -> Link {
    let mut head = None;

    for value in values.into_iter().rev() {
        let mut node = Box::new(ListNode::new(value));
        node.next = head;
        head = Some(node);
    }

    head
}

/// Convert a linked list to values.
///
/// Time: O(n)
/// Space: O(n)
pub fn list_to_vec(head: &Link) -> Vec<i32> {
    let mut values = Vec::new();
    let mut current = head.as_ref();

    while let Some(node) = current {
        values.push(node.val);
        current = node.next.as_ref();
    }

    values
}

/// Reverse Linked List
///
/// Pattern: iterative pointer rewiring.
/// Idea: move nodes from the original list to the front of a reversed list.
///
/// Time: O(n)
/// Space: O(1)
pub fn reverse_list(mut head: Link) -> Link {
    let mut previous = None;

    while let Some(mut node) = head {
        head = node.next.take();
        node.next = previous;
        previous = Some(node);
    }

    previous
}

/// Merge Two Sorted Lists
///
/// Pattern: recursion over the smaller head.
/// Idea: choose the smaller current node and merge the rest.
///
/// Time: O(n + m)
/// Space: O(n + m) recursion depth.
pub fn merge_two_lists(left: Link, right: Link) -> Link {
    match (left, right) {
        (None, None) => None,
        (Some(node), None) | (None, Some(node)) => Some(node),
        (Some(mut left_node), Some(mut right_node)) => {
            if left_node.val <= right_node.val {
                let next = left_node.next.take();
                left_node.next = merge_two_lists(next, Some(right_node));
                Some(left_node)
            } else {
                let next = right_node.next.take();
                right_node.next = merge_two_lists(Some(left_node), next);
                Some(right_node)
            }
        }
    }
}

/// Linked List Cycle
///
/// Pattern: fast/slow pointers.
/// Idea: use next-index representation to model cycles safely without unsafe references.
///
/// Time: O(n)
/// Space: O(1)
pub fn has_cycle(next: Vec<Option<usize>>) -> bool {
    if next.is_empty() {
        return false;
    }

    let mut slow = Some(0);
    let mut fast = Some(0);

    loop {
        slow = advance(&next, slow, 1);
        fast = advance(&next, fast, 2);

        match (slow, fast) {
            (Some(slow_index), Some(fast_index)) if slow_index == fast_index => return true,
            (None, _) | (_, None) => return false,
            _ => {}
        }
    }
}

fn advance(next: &[Option<usize>], mut current: Option<usize>, steps: usize) -> Option<usize> {
    for _ in 0..steps {
        let index = current?;
        current = next.get(index).copied().flatten();
    }

    current
}

/// Reorder List
///
/// Pattern: front/back interleaving.
/// Idea: collect values, then rebuild in L0, Ln, L1, Ln-1 order.
///
/// Time: O(n)
/// Space: O(n)
pub fn reorder_list(head: Link) -> Link {
    let values = list_to_vec(&head);
    let mut reordered = Vec::with_capacity(values.len());

    if values.is_empty() {
        return None;
    }

    let mut left = 0;
    let mut right = values.len() - 1;

    while left < right {
        reordered.push(values[left]);
        reordered.push(values[right]);
        left += 1;
        right -= 1;
    }

    if left == right {
        reordered.push(values[left]);
    }

    list_from_vec(reordered)
}
