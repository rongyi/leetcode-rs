struct Solution;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

use std::collections::BinaryHeap;
#[derive(Eq, PartialEq)]
struct NodeWrapper(Box<ListNode>);

impl Ord for NodeWrapper {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.0.val.cmp(&self.0.val)
    }
}

impl PartialOrd for NodeWrapper {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(other.0.val.cmp(&self.0.val))
    }
}

impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode::new(-1));
        let mut tail = &mut dummy;
        let mut heap = BinaryHeap::new();

        for l in lists.into_iter() {
            if let Some(cur) = l {
                heap.push(NodeWrapper(cur));
            }
        }
        while let Some(mut cur_node_wrapper) = heap.pop() {
            if let Some(next) = cur_node_wrapper.0.next.take() {
                heap.push(NodeWrapper(next));
            }
            tail.next = Some(cur_node_wrapper.0);
            tail = tail.next.as_mut().unwrap();
        }

        dummy.next.take()
    }
}

fn main() {}
