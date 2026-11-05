
struct Solution;

use std::collections::HashSet;
impl Solution {
    pub fn subarray_bitwise_o_rs(arr: Vec<i32>) -> i32 {
        let mut ret: HashSet<i32> = HashSet::new();
        let mut prev = HashSet::new();

        for &num in arr.iter() {
            let mut cur = HashSet::new();

            cur.insert(num);

            for &p in prev.iter() {
                cur.insert(p | num);
            }
            ret.extend(cur.iter());
            prev = cur;
        }

        ret.len() as i32
    }
}

fn main() {}
