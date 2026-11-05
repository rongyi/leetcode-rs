
struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn largest_vals_from_labels(
        values: Vec<i32>,
        labels: Vec<i32>,
        num_wanted: i32,
        use_limit: i32,
    ) -> i32 {
        let mut pairs: Vec<(i32, i32)> = values.into_iter().zip(labels.into_iter()).collect();
        pairs.sort_unstable_by(|a, b| b.0.cmp(&a.0));

        let mut label_cnt: HashMap<i32, i32> = HashMap::new();
        let mut selected = 0;
        let mut sum = 0;

        for (value, label) in pairs {
            let cnt = label_cnt.entry(label).or_insert(0);

            if selected < num_wanted && *cnt < use_limit {
                sum += value;
                selected += 1;
                *cnt += 1;
            }
        }

        sum
    }
}

fn main() {}
