#![allow(dead_code)]
struct Solution;

impl Solution {
    // 看图
    // https://leetcode.com/problems/minimum-adjacent-swaps-for-k-consecutive-ones/discuss/987607/O(n)-explanation-with-picture
    pub fn min_moves(nums: Vec<i32>, k: i32) -> i32 {
        let sz = nums.len();
        let mut ones: Vec<usize> = Vec::new();
        for (i, &val) in nums.iter().enumerate() {
            if val == 1 {
                ones.push(i);
            }
        }

        let mut prefix = vec![0i64; sz + 1];
        let mut ret = i64::MAX;
        let mut i = 0;
        for j in 0..ones.len() {
            prefix[j + 1] = prefix[j] + ones[j] as i64;

            if j + 1 < k as usize {
                continue;
            }
            let mid: i64 = (i + (j - i) / 2) as i64;
            let mut move_cnt = 0;
            let mut save: i64 = 0;
            if k % 2 == 1 {
                move_cnt =
                    prefix[j + 1] - prefix[mid as usize + 1] - prefix[mid as usize] + prefix[i];
                save = (mid - i as i64 + 1) * (mid - i as i64); // radius * (radius + 1)
            } else {
                move_cnt =
                    prefix[j + 1] - prefix[mid as usize + 1] - prefix[mid as usize + 1] + prefix[i];
                save = (mid - i as i64 + 1) * (mid - i as i64 + 1);
            }

            ret = ret.min(move_cnt - save);
            i += 1;
        }

        ret as i32
    }
}

fn main() {}
