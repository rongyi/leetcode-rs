struct Solution;

impl Solution {
    pub fn min_k_bit_flips(nums: Vec<i32>, k: i32) -> i32 {
        let sz = nums.len();
        let mut flipped = vec![0; sz];
        let mut cur_flip = 0;
        let mut ret = 0;
        let k = k as usize;
        for i in 0..sz {
            if i >= k {
                cur_flip ^= flipped[i - k];
            }

            if cur_flip == nums[i] {
                if i + k > sz {
                    return -1;
                }
                flipped[i] = 1;
                cur_flip ^= 1;
                ret += 1;
            }
        }

        ret
    }
}

fn main() {}
