struct Solution;

use std::collections::BTreeSet;
impl Solution {
    pub fn second_greater_element(nums: Vec<i32>) -> Vec<i32> {
        let sz = nums.len();
        let mut order = (0..sz).collect::<Vec<_>>();
        order.sort_by_key(|&i| -nums[i]);
        let mut ret = vec![-1; sz];

        let mut cache: BTreeSet<usize> = BTreeSet::new();

        for i in order {
            for (k, &j) in cache.range(i + 1..).enumerate() {
                if k == 1 {
                    ret[i] = nums[j];
                    break;
                }
            }
            cache.insert(i);
        }

        ret
    }
}

fn main() {}
