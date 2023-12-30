// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
                let mut carry = 0;
        let mut l1 = l1;
        let mut l2 = l2;
        let mut dummy = ListNode::new(-1);
        let mut p = &mut dummy;
        while l1.is_some() || l2.is_some() {
            let mut sum = carry;
            if let Some(node) = l1 {
                sum += node.val;
                l1 = node.next;
            }
            if let Some(node) = l2 {
                sum += node.val;
                l2 = node.next;
            }
            carry = sum / 10;
            p.next = Some(Box::new(ListNode {
                val: sum % 10,
                next: None,
            }));
            p = p.next.as_mut().unwrap();
        }
                if carry != 0 {
            p.next = Some(Box::new(ListNode { val: carry, next: None }));
        }

        dummy.next

    }
}
