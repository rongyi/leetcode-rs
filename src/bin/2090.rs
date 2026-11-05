struct Solution;

impl Solution {
    pub fn get_averages(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let sz = nums.len();
        let mut wing_sum = 0i64;
        let mut ret = vec![];

        for i in 0..(2 * k as usize).min(sz) {
            wing_sum += nums[i] as i64;
        }

        for i in 0..sz {
            let l = i as i32 - k;
            let r = i + k as usize;
            if l < 0 || r >= sz {
                ret.push(-1);
            } else {
                wing_sum += nums[i + k as usize] as i64;
                ret.push((wing_sum / (2 * k as i64 + 1)) as i32);
                wing_sum -= nums[i - k as usize] as i64;
            }
        }

        ret
    }
}

fn main() {}
