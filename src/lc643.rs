struct Solution;

impl Solution {
    pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
        let ku = k as usize;
        let kf = k as f64;
        let nums: Vec<f64> = nums.into_iter().map(|i| i as f64).collect();
        let mut max_avg = f64::MIN;

        let mut win_sum = 0.0;
        for (i, &num) in nums.iter().enumerate() {
            win_sum += num;
            // if i >= ku {
            //     win_sum -= nums[i - ku];
            // }

            if i >= ku - 1 {
                max_avg = max_avg.max(win_sum / kf);
                // we can cut the window end using a if check above, also
                // can be don here, with one shif, because we do it in one
                // step early
                win_sum -= nums[i - ku + 1];
            }
        }

        max_avg
    }
}

fn main() {}
