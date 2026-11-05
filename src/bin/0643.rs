struct Solution;

impl Solution {
    pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
        let ku = k as usize;
        let kf = k as f64;
        let nums: Vec<f64> = nums.into_iter().map(|i| i as f64).collect();

        let mut win_sum = 0.0;
        for i in 0..ku {
            win_sum += nums[i];
        }
        let mut max_avg = win_sum / kf;

        for (i, &num) in nums.iter().enumerate().skip(ku) {
            win_sum += num;
            win_sum -= nums[i - ku];
            max_avg = max_avg.max(win_sum / kf);
        }

        max_avg
    }
}

fn main() {}
