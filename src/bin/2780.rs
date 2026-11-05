struct Solution;

impl Solution {
    pub fn minimum_index(nums: Vec<i32>) -> i32 {
        let mut major = 0;
        let mut cnt = 0;
        // find the major
        for &num in nums.iter() {
            if cnt == 0 {
                major = num;
                cnt = 1;
            } else if num == major {
                cnt += 1;
            } else {
                cnt -= 1;
            }
        }
        // count total:
        let total = nums.iter().filter(|x| **x == major).count();

        let mut left_part = 0;
        for i in 0..nums.len() - 1 {
            if nums[i] == major {
                left_part += 1;
            }
            let right_part = total - left_part;
            let left_len = i + 1;
            let right_len = nums.len() - left_len;
            if left_part * 2 > left_len && right_part * 2 > right_len {
                return i as _;
            }
        }

        -1
    }
}

fn main() {}
