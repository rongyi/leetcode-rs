struct Solution;

use rand::{thread_rng, Rng};

struct Solution {
    head: Option<Box<ListNode>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {
    fn new(head: Option<Box<ListNode>>) -> Self {
        Solution { head }
    }

    fn get_random(&self) -> i32 {
        let mut rng = thread_rng();
        let mut ret = 0;
        let mut cnt = 0;
        let mut cur = self.head.as_ref();
        while let Some(node) = cur {
            if rng.gen_range(0..=cnt) == 0 {
                ret = node.val;
            }

            cur = node.next.as_ref();
            cnt += 1;
        }

        ret
    }
}

fn main() {}
