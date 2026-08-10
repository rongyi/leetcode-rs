struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn find_shortest_sub_array(nums: Vec<i32>) -> i32 {
        // Map stores: num -> (count, first_index, last_index)
        let mut map: HashMap<i32, (usize, usize, usize)> = HashMap::new();
        let mut max_degree = 0;

        for (i, &num) in nums.iter().enumerate() {
            let entry = map
                .entry(num)
                .and_modify(|e| {
                    e.0 += 1;
                    e.2 = i;
                })
                .or_insert((1, i, i));

            max_degree = max_degree.max(entry.0);
        }

        let mut min_len = nums.len();

        for (count, first, last) in map.values() {
            if *count == max_degree {
                min_len = min_len.min(last - first + 1);
            }
        }

        min_len as i32
    }
}

fn main() {}
