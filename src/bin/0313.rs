struct Solution;

use std::{cmp::Reverse, collections::BinaryHeap};

impl Solution {
    pub fn nth_super_ugly_number(n: i32, primes: Vec<i32>) -> i32 {
        let mut q = BinaryHeap::new();
        q.push(Reverse(1));
        let mut ret = 0;
        let mut cnt = 0;
        while !q.is_empty() && cnt < n {
            let cur = q.pop().unwrap().0;
            ret = cur;

            for &p in primes.iter() {
                let next_candidate = p as i64 * cur;
                q.push(Reverse(next_candidate));
            }

            // clear this round
            while !q.is_empty() && q.peek().unwrap().0 == cur {
                q.pop();
            }

            cnt += 1;
        }

        ret as _
    }
}

fn main() {}
