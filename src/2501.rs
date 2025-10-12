struct Solution;

use std::collections::BTreeSet;
impl Solution {
    pub fn longest_square_streak(nums: Vec<i32>) -> i32 {
        let mut uniq: BTreeSet<i64> = nums.iter().fold(BTreeSet::new(), |mut acc, cur| {
            acc.insert(*cur as i64);
            acc
        });

        let mut ret = -1;
        for &num in nums.iter() {
            ret = ret.max(Self::lss(num as i64, &uniq));
        }
        ret
    }
    fn lss(mut num: i64, uniq: &BTreeSet<i64>) -> i32 {
        let mut len = 0;
        while uniq.contains(&num) {
            num *= num;
            len += 1;
        }
        if len < 2 {
            return -1;
        }
        len
    }
}

fn main() {}
