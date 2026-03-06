struct Solution;

use std::collections::VecDeque;
impl Solution {
    pub fn lexicographically_smallest_array(nums: Vec<i32>, limit: i32) -> Vec<i32> {
        let mut sz = nums.len();
        let mut ret = vec![0; sz];
        let mut nums_with_idx: Vec<(i32, usize)> =
            nums.into_iter().enumerate().map(|k| (k.1, k.0)).collect();
        nums_with_idx.sort_by_key(|x| x.0);

        let mut cur_group = VecDeque::new();
        cur_group.push_back(nums_with_idx[0].0);
        let mut all_group = vec![];
        let mut id_to_group = vec![0; sz];
        id_to_group[nums_with_idx[0].1] = 0;

        for i in 1..sz {
            if nums_with_idx[i].0 - nums_with_idx[i - 1].0 > limit {
                all_group.push(cur_group);
                cur_group = VecDeque::new();
            }
            let group_id = all_group.len();
            id_to_group[nums_with_idx[i].1] = group_id;
            cur_group.push_back(nums_with_idx[i].0);
        }
        all_group.push(cur_group);

        for i in 0..sz {
            ret[i] = all_group[id_to_group[i]].pop_front().unwrap();
        }

        ret
    }
}

fn main() {}
