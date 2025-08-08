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
    pub fn pair_sum(head: Option<Box<ListNode>>) -> i32 {
        let mut out: Vec<i32> = vec![];
        Self::to_vec(head.as_ref(), &mut out);
        let sz = out.len();
        let mut max_sum = 0;
        for (i, j) in (0..sz / 2).rev().zip(sz / 2..sz) {
            max_sum = max_sum.max(out[i] + out[j]);
        }

        max_sum
    }
    fn to_vec(head: Option<&Box<ListNode>>, out: &mut Vec<i32>) {
        if let Some(b) = head {
            out.push(b.val);
            Self::to_vec(b.next.as_ref(), out);
        };
    }
}

fn main() {}
