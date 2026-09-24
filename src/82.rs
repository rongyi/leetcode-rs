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
    pub fn delete_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode::new(-1));
        let mut tail = &mut dummy;

        while let Some(mut cur) = head {
            let mut duplicate = false;
            while let Some(next) = cur.next.as_ref() {
                if cur.val == next.val {
                    duplicate = true;
                    cur = cur.next.unwrap();
                } else {
                    break;
                }
            }

            head = cur.next.take();

            if !duplicate {
                tail.next = Some(cur);
                tail = tail.next.as_mut().unwrap();
            }
        }

        dummy.next
    }
}
fn main() {}
