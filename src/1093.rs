struct Solution;

impl Solution {
    pub fn sample_stats(count: Vec<i32>) -> Vec<f64> {
        let mut total_count: i64 = 0;
        let mut sum: i64 = 0;
        let mut min: i32 = 256;
        let mut max: i32 = 0;
        let mut mode: i32 = 0;
        let mut mode_count: i32 = 0;

        // Calculate min, max, sum, and mode
        for i in 0..count.len() {
            if count[i] > 0 {
                min = min.min(i as i32);
                max = max.max(i as i32);
                sum += (i as i64) * (count[i] as i64);
                total_count += count[i] as i64;

                if count[i] > mode_count {
                    mode_count = count[i];
                    mode = i as i32;
                }
            }
        }

        // Calculate median
        let mut median: f64 = 0.0;
        let mid = total_count + 1;
        let mut count_sum: i64 = 0;
        let mut prev: i32 = 0;

        if total_count % 2 == 0 {
            let target1 = total_count / 2;
            let target2 = target1 + 1;
            let mut found1 = false;
            let mut val1 = 0;
            let mut val2 = 0;

            for i in 0..count.len() {
                count_sum += count[i] as i64;
                if !found1 && count_sum >= target1 {
                    val1 = i as i32;
                    found1 = true;
                }
                if count_sum >= target2 {
                    val2 = i as i32;
                    break;
                }
            }
            median = (val1 + val2) as f64 / 2.0;
        } else {
            let target = (total_count + 1) / 2;
            for i in 0..count.len() {
                count_sum += count[i] as i64;
                if count_sum >= target {
                    median = i as f64;
                    break;
                }
            }
        }

        vec![
            min as f64,
            max as f64,
            sum as f64 / total_count as f64,
            median,
            mode as f64,
        ]
    }
}

fn main() {}
