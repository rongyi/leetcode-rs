struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn is_possible(nums: Vec<i32>) -> bool {
        let mut cnt: HashMap<i32, i32> = HashMap::new();

        for &num in nums.iter() {
            *cnt.entry(num).or_insert(0) += 1;
        }
        let mut tail: HashMap<i32, i32> = HashMap::new();
        for &num in nums.iter() {
            if cnt[&num] <= 0 {
                continue;
            }
            cnt.entry(num).and_modify(|c| *c -= 1);
            if *tail.get(&(num - 1)).unwrap_or(&0) > 0 {
                *tail.entry(num - 1).or_insert(0) -= 1;
                *tail.entry(num).or_insert(0) += 1;
            } else if *cnt.get(&(num + 1)).unwrap_or(&0) > 0
                && *cnt.get(&(num + 2)).unwrap_or(&0) > 0
            {
                *cnt.entry(num + 1).or_insert(0) -= 1;
                *cnt.entry(num + 2).or_insert(0) -= 1;
                *tail.entry(num + 2).or_insert(0) += 1;
            } else {
                return false;
            }
        }

        true
    }
}

fn main() {
    let mut cnt: HashMap<i32, i32> = HashMap::new();
    cnt.insert(1, 10);
    cnt.entry(1).and_modify(|c| *c += 1);
    println!("{:?}", cnt);
}
