struct Solution;

use std::{cmp::Reverse, collections::BinaryHeap};
impl Solution {
    pub fn maximum_bags(capacity: Vec<i32>, rocks: Vec<i32>, mut additional_rocks: i32) -> i32 {
        // smaller first
        let mut q: BinaryHeap<Reverse<i32>> = BinaryHeap::new();
        let mut already_full = 0;
        let mut refill_full = 0;

        for i in 0..capacity.len() {
            let need_rocks = capacity[i] - rocks[i];
            if need_rocks <= 0 {
                already_full += 1;
            } else {
                q.push(Reverse(need_rocks));
            }
        }

        while let Some(Reverse(cur_need)) = q.pop() {
            println!("cur need: {}", cur_need);
            if cur_need <= additional_rocks {
                refill_full += 1;
                additional_rocks -= cur_need;
            } else {
                // we can assure latter can not fufilled
                break;
            }
        }

        already_full + refill_full
    }
}

fn main() {
    let input1 = vec![2, 3, 4, 5];
    let input2 = vec![1, 2, 4, 4];
    Solution::maximum_bags(input1, input2, 2);
}
