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

impl Solution {
    pub fn next_larger_nodes(mut head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut lst = Vec::new();
        while let Some(mut node) = head {
            lst.push(node.val.clone());
            // consume this single list
            head = node.next.take();
        }
        let mut stk = Vec::new();
        let mut ret = Vec::new();
        for &val in lst.iter().rev() {
            while !stk.is_empty() && *stk.last().unwrap() <= val {
                stk.pop();
            }
            ret.push(*stk.last().unwrap_or(&0));
            stk.push(val);
        }
        ret.into_iter().rev().collect()
    }
}

fn main() {}
