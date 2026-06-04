struct Solution;

impl Solution {
    pub fn maximum_gap(nums: Vec<i32>) -> i32 {
        let sz = nums.len();
        if sz <= 1 {
            return 0;
        }
        let mut max_val = nums.iter().max().unwrap().to_owned();
        let mut min_val = nums.iter().min().unwrap().to_owned();
        if max_val == min_val {
            return 0;
        }
        let mut gap = max_val - min_val;
        let mut bucket_size = std::cmp::max(gap / (sz as i32 - 1), 1);
        let mut bucket_count = 1 + gap / bucket_size;

        let mut group: Vec<Option<(i32, i32)>> = vec![None; bucket_count as usize];

        for &num in nums.iter() {
            let idx = ((num - min_val) / bucket_size) as usize;
            if let Some((bkt_min, bkt_max)) = group[idx] {
                group[idx] = Some((bkt_min.min(num), (bkt_max.max(num))));
            } else {
                group[idx] = Some((num, num));
            }
        }
        let mut prev_max = min_val;
        let mut ret = 0;
        for g in group.iter() {
            if let Some(v) = g {
                let cur_gap = v.0 - prev_max;
                ret = ret.max(cur_gap);
                prev_max = v.1;
            }
        }

        ret
    }
}

fn main() {}
