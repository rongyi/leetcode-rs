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
    pub fn reverse_between(
        mut head: Option<Box<ListNode>>,
        left: i32,
        right: i32,
    ) -> Option<Box<ListNode>> {
        // shortcut
        if left == right {
            return head;
        }

        let mut dummy = Box::new(ListNode::new(-1));
        let mut tail = &mut dummy;
        let mut p1 = head;
        let mut cur_idx = 1;

        let mut d2 = Box::new(ListNode::new(-1));
        let mut rtail = &mut d2;

        loop {
            match p1 {
                None => {
                    break;
                }
                Some(mut cur) => {
                    let next = cur.next.take();
                    if cur_idx >= left && cur_idx <= right {
                        cur.next = rtail.next.take();
                        rtail.next = Some(cur);
                    } else {
                        tail.next = Some(cur);
                        tail = tail.next.as_mut().unwrap();
                    }
                    p1 = next;

                    if cur_idx == right {
                        tail.next = rtail.next.take();
                        while tail.next.is_some() {
                            tail = tail.next.as_mut().unwrap();
                        }
                    }
                    cur_idx += 1;
                }
            }
        }

        dummy.next.take()
    }
}

fn main() {}
