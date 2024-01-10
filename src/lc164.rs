struct Solution;

impl Solution {
    pub fn maximum_gap(nums: Vec<i32>) -> i32 {
        let sz = nums.len() as i32;
        if sz < 2 {
            return 0;
        }
        let l = nums.iter().min().unwrap().to_owned();
        let r = nums.iter().max().unwrap().to_owned();

        let min_gap = ((r - l) / (sz - 1)).max(1);
        let bucket_nums = (1 + (r - l) / min_gap) as usize;

        let mut bmins = vec![i32::MAX; bucket_nums];
        let mut bmaxs = vec![i32::MIN; bucket_nums];

        for i in 0..nums.len() {
            let idx = ((nums[i] - l) / min_gap) as usize;
            bmins[idx] = bmins[idx].min(nums[i]);
            bmaxs[idx] = bmaxs[idx].max(nums[i]);
        }
        let mut ret = 0;
        let mut prev_max = l;

        for i in 0..bucket_nums {
            if bmins[i] == i32::MAX || bmaxs[i] == i32::MIN {
                continue;
            }
            ret = ret.max(bmins[i] - prev_max);
            prev_max = bmaxs[i];
        }

        ret
    }
}

fn main() {}
