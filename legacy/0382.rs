// import external C code

// yes we can use rand in leetcode rust
use rand::{thread_rng, Rng};
extern "C" {
    fn srand() -> u32;
    fn rand() -> u32;
}

// random number function
fn my_rand_number() -> u32 {
    unsafe {
        srand();
        rand()
    }
}

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

/**
 * Your Solution object will be instantiated and called as such:
 * let obj = Solution::new(head);
 * let ret_1: i32 = obj.get_random();
 */

fn main() {}
