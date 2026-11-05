struct Solution;

impl Solution {
    pub fn maximum_strong_pair_xor(nums: Vec<i32>) -> i32 {
        let sz = nums.len();
        let mut max_xor = 0;
        for i in 0..sz {
            for j in i + 1..sz {
                let x = nums[i];
                let y = nums[j];
                if (x - y).abs() <= x.min(y) {
                    if x ^ y > max_xor {
                        max_xor = x ^ y;
                    }
                }
            }
        }
        max_xor
    }
}

fn main() {}
