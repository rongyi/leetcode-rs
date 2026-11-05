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
    pub fn insertion_sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = Some(Box::new(ListNode {
            val: -1,
            next: head,
        }));
        let mut last_sorted = &mut dummy.as_mut().unwrap().next as *mut Option<Box<ListNode>>;

        unsafe {
            while (*last_sorted).as_ref().unwrap().next.is_some() {
                if (*last_sorted).as_ref().unwrap().val
                    <= (*last_sorted).as_ref().unwrap().next.as_ref().unwrap().val
                {
                    last_sorted = &mut (*last_sorted).as_mut().unwrap().next as *mut _;
                } else {
                    let mut unsorted = (*last_sorted).as_mut().unwrap().next.take();
                    (*last_sorted).as_mut().unwrap().next = unsorted.as_mut().unwrap().next.take();

                    let mut ptr = &mut dummy as *mut Option<Box<ListNode>>;
                    while (*ptr).as_ref().unwrap().next.as_ref().unwrap().val
                        <= unsorted.as_ref().unwrap().val
                    {
                        ptr = &mut (*ptr).as_mut().unwrap().next as *mut _;
                    }

                    unsorted.as_mut().unwrap().next = (*ptr).as_mut().unwrap().next.take();
                    (*ptr).as_mut().unwrap().next = unsorted;
                }
            }
        }

        dummy.unwrap().next
    }
}

fn main() {}
