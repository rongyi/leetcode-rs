#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn best_rotation(nums: Vec<i32>) -> i32 {
        let sz = nums.len();
        let mut change = vec![0; sz];
        for i in 0..sz {
            change[(i - nums[i] as usize + sz + 1) % sz] -= 1;
        }

        for i in 1..sz {
            change[i] += change[i - 1] + 1;
        }

        let mut max_num = i32::MIN;
        let mut max_idx = 0;
        for (i, num) in change.into_iter().enumerate() {
            if num > max_num {
                max_idx = i;
                max_num = num;
            }
        }
        max_idx as i32
    }
}

fn main() {
    let input = vec![1, 1, 1];
}
