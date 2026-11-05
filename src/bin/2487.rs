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
    pub fn remove_nodes(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        Self::build_dfs(head.as_ref())
    }
    fn build_dfs(head: Option<&Box<ListNode>>) -> Option<Box<ListNode>> {
        match head {
            None => None,
            Some(cur) => {
                let rest = Self::build_dfs(cur.next.as_ref());
                match rest.as_ref() {
                    None => return head.map(|c| c.to_owned()),
                    Some(r) => {
                        // shadow the value
                        if cur.val < r.val {
                            return rest;
                        }
                        Some(Box::new(ListNode {
                            val: cur.val,
                            next: rest,
                        }))
                    }
                }
            }
        }
    }
}

fn main() {}
