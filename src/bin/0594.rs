struct Solution;

use std::collections::BTreeMap;
impl Solution {
    pub fn find_lhs(nums: Vec<i32>) -> i32 {
        let mut cnt: BTreeMap<i32, i32> = BTreeMap::new();
        for num in nums.into_iter() {
            *cnt.entry(num).or_insert(0) += 1;
        }
        let mut ret = 0;
        for (&num, &occ) in cnt.iter() {
            if let Some(&val) = cnt.get(&(num + 1)) {
                ret = ret.max(occ + val);
            }
        }

        ret
    }
}

fn main() {}
