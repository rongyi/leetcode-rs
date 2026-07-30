struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn least_interval(tasks: Vec<char>, n: i32) -> i32 {
        let mut freq: HashMap<char, i32> = HashMap::new();
        for &c in tasks.iter() {
            *freq.entry(c).or_insert(0) += 1;
        }
        let max_freq = *freq.values().max().unwrap();
        let max_occ_cnt = freq.values().filter(|&&c| c == max_freq).count() as i32;

        let slot_cnt = max_freq - 1;
        // exclude the leading one
        let slot_len = n - (max_occ_cnt - 1);
        let empty_slots = slot_cnt * slot_len;
        let available_tasks = tasks.len() as i32 - max_freq * max_occ_cnt;
        let idles = 0.max(empty_slots - available_tasks);

        tasks.len() as i32 + idles
    }
}

fn main() {}
