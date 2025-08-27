
struct Solution;

use std::collections::HashSet;
impl Solution {
    pub fn count_distinct(nums: Vec<i32>, k: i32, p: i32) -> i32 {
        let mut uniq: HashSet<Vec<i32>> = HashSet::new();
        let sz = nums.len();
        for i in 0..sz {
            let mut cur = vec![];
            let mut cnt = 0;
            for j in i..sz {
                cur.push(nums[j]);
                if nums[j] % p == 0 {
                    cnt += 1;
                }
                if cnt > k {
                    break;
                }
                uniq.insert(cur.clone());
            }
        }
        uniq.len() as _
    }
}

fn main() {}
