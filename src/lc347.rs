
struct Solution;
use std::collections::{BinaryHeap, HashMap};
impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut ret = Vec::new();
        let mut cnt = HashMap::new();
        for &num in &nums {
            *cnt.entry(num).or_insert(0) += 1;
        }
        let mut q = BinaryHeap::new();
        for (&num, &freq) in cnt.iter() {
            q.push((freq, num));
        }
        let mut k = k;
        while !q.is_empty() && k != 0 {
            if let Some((freq, num)) = q.pop() {
                ret.push(num);
            }
            k -= 1;
        }

        ret
    }
}

fn main() {}
