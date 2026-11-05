struct Solution;

impl Solution {
    pub fn partition_disjoint(nums: Vec<i32>) -> i32 {
        let sz = nums.len();
        let mut cur_min = vec![i32::MAX; sz];
        let mut cur_max = vec![-1; sz];

        let mut tmp = -1;
        for i in 0..sz {
            tmp = tmp.max(nums[i]);
            cur_max[i] = tmp;
        }

        tmp = i32::MAX;
        for i in (0..sz).rev() {
            tmp = tmp.min(nums[i]);
            cur_min[i] = tmp;
        }

        for i in 0..sz - 1 {
            if cur_max[i] <= cur_min[i + 1] {
                return (i + 1) as i32;
            }
        }

        sz as i32
    }
}

fn main() {}
