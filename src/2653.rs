struct Solution;

use std::collections::BTreeMap;
impl Solution {
    pub fn get_subarray_beauty(nums: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
        let mut cnt: BTreeMap<i32, i32> = BTreeMap::new();
        let k = k as usize;
        let mut ret = vec![];
        for (i, &num) in nums.iter().enumerate() {
            *cnt.entry(num).or_default() += 1;
            if i >= k - 1 {
                if i >= k {
                    cnt.entry(nums[i - k]).and_modify(|v| *v -= 1);
                }
                let mut cur_cnt = 0;
                for (k, v) in cnt.iter() {
                    // check key
                    if *k < 0 {
                        cur_cnt += v;
                        if cur_cnt >= x {
                            ret.push(*k);
                            break;
                        }
                    } else {
                        ret.push(0);
                        break;
                    }
                }
            }
        }

        ret
    }
}

fn main() {}
