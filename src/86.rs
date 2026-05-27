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
    pub fn partition(head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
        let mut d1 = Box::new(ListNode::new(-1));
        let mut d2 = Box::new(ListNode::new(-1));
        let mut t1 = &mut d1;
        let mut t2 = &mut d2;
        let mut cur = head;
        loop {
            match cur {
                None => {
                    break;
                }
                Some(mut p) => {
                    let next = p.next.take();
                    if p.val < x {
                        t1.next = Some(p);
                        t1 = t1.next.as_mut().unwrap();
                    } else {
                        t2.next = Some(p);
                        t2 = t2.next.as_mut().unwrap();
                    }
                    cur = next;
                }
            }
        }

        t1.next = d2.next.take();

        d1.next.take()
    }
}

fn main() {}
