
struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn repeated_n_times(mut nums: Vec<i32>) -> i32 {
        let mut cnt: HashMap<i32, i32> = HashMap::new();

        for &num in nums.iter() {
            let e = cnt.entry(num).or_insert(0);
            *e += 1;
            if *e >= nums.len() as i32 / 2 {
                return num;
            }
        }
        unreachable!()
    }
}

fn main() {}
