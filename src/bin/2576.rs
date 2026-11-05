struct Solution;

impl Solution {
    pub fn max_num_of_marked_indices(mut nums: Vec<i32>) -> i32 {
        let sz = nums.len();
        if sz < 2 {
            return 0;
        }
        nums.sort_unstable();
        let mut ret = 0;
        let mut l = 0;
        let mut r = sz / 2;
        while l < sz / 2 && r < sz {
            let v1 = nums[l] * 2;
            let v2 = nums[r];
            if v1 <= v2 {
                ret += 2;
                l += 1;
                r += 1;
                continue;
            }
            // simply add one
            r += 1;
        }

        ret
    }
}

fn main() {}
