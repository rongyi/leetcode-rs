struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn can_reorder_doubled(arr: Vec<i32>) -> bool {
        let mut count: HashMap<i32, i32> = HashMap::new();
        for &num in arr.iter() {
            *count.entry(num).or_insert(0) += 1;
        }

        let mut nums: Vec<i32> = count.keys().copied().collect();
        nums.sort_by_key(|&x| x.abs());

        for &num in nums.iter() {
            match count.get(&num) {
                Some(&cur) if cur == 0 => {
                    continue;
                }
                Some(&cur) => {
                    let pair_key = num * 2;
                    if *count.get(&pair_key).unwrap_or(&0) < cur {
                        return false;
                    }

                    count.entry(num * 2).and_modify(|v| *v -= cur);
                }
                None => continue,
            }
        }

        true
    }
}

fn main() {}
