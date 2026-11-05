struct Solution;
use std::collections::BinaryHeap;

impl Solution {
    // ugly number                       k sorted list
    //     1                            2     7    13   19     1 * [2,7,13,19]
    //     |                            |     |    |    |
    //     2                            4     14   26   38     2 * [2,7,13,19]
    //     |                            |     |    |    |
    //     4                            8     28   52   76     4 * [2,7,13,19]
    //     |                            |     |    |    |
    //     7                            14    49   91   133    7 * [2,7,13,19]
    //     |                            |     |    |    |
    //     8                            16    56   ...   ...   8 * [2,7,13,19]
    //     |                            |     |    |     |
    //     .                            .     .     .    .
    //     .                            .     .     .    .
    //     .                            .     .     .    .
    pub fn nth_super_ugly_number(n: i32, primes: Vec<i32>) -> i32 {
        let mut q = BinaryHeap::new();
        q.push(std::cmp::Reverse(1i64));
        let mut ret = 0;
        let mut cnt = 0;
        while !q.is_empty() && cnt < n {
            let cur = q.pop().unwrap().0;
            ret = cur;

            for &p in &primes {
                q.push(std::cmp::Reverse((cur as i64) * (p as i64)));
            }
            while !q.is_empty() && q.peek().unwrap().0 == cur {
                q.pop();
            }

            cnt += 1;
        }

        ret as i32
    }
}

fn main() {}
