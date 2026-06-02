struct Solution;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl Solution {
    pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
        if head.is_none() || head.as_mut().unwrap().next.is_none() {
            return;
        }
        let mut len = 0;
        let mut cur = head.as_ref();
        while let Some(n) = cur {
            cur = n.next.as_ref();
            len += 1;
        }
        let mut cur = head.as_mut();
        for _ in 0..(len + 1) / 2 - 1 {
            cur = cur.unwrap().next.as_mut();
        }
        let mut bottom_half = cur.as_mut().unwrap().next.take();
        // 2. Reverse the second half
        let mut prev = None;
        let mut curr = bottom_half;
        while let Some(mut node) = curr {
            let next = node.next.take();
            node.next = prev;
            prev = Some(node);
            curr = next;
        }
        let mut bottom_half = prev;

        let mut bottom_turn = true;

        let mut up_half = head.as_mut().unwrap().next.take();
        let mut cur = head.as_mut().unwrap();

        while bottom_half.is_some() || up_half.is_some() {
            if bottom_turn {
                cur.next = bottom_half;
                bottom_half = cur.next.as_mut().unwrap().next.take();
            } else {
                cur.next = up_half;
                up_half = cur.next.as_mut().unwrap().next.take();
            }
            cur = cur.next.as_mut().unwrap();
            bottom_turn = !bottom_turn;
        }
    }
}

fn main() {}
