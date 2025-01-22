#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn max_value_after_reverse(nums: Vec<i32>) -> i32 {
        let sz = nums.len();
        let mut sum = 0;
        let mut min_max_ab = i32::MAX;
        let mut max_min_cd = i32::MIN;
        let mut ret = 0;

        // case1: a [b...c] d, i.e. "inner" subarray
        // length of the yellow line = min(c,d) - max(a,b)
        for i in 1..sz {
            sum += (nums[i] - nums[i - 1]).abs();
            // Notice length of the yellow line = min(c,d) - max(a,b), we only need to find out max(min(c,d) for any c,d)
            //  and min(max(a,b) for any a,b)). An iteration is enough.
            min_max_ab = min_max_ab.min(nums[i].max(nums[i - 1]));
            max_min_cd = max_min_cd.max(nums[i].min(nums[i - 1]));
        }
        ret = sum + ((max_min_cd - min_max_ab) * 2).max(0);

        // case2: subarray start with 0 or end with n - 1
        // swap from i to j where i,j are either 0 or n-1: Assuming that i is 0 then
        // what we gain is |a[0]-a[j+1]| - |a[j]-a[j+1]|. Scan a second time to
        // calculate max gain.
        let mut t1 = 0;
        let mut t2 = 0;
        for i in 1..sz - 1 {
            // reverse subarray[0..i]
            t1 = t1.max((nums[i + 1] - nums[0]).abs() - (nums[i + 1] - nums[i]).abs());
        }

        for i in 0..sz - 1 {
            t2 = t2.max((nums[sz - 1] - nums[i]).abs() - (nums[i + 1] - nums[i]).abs());
        }

        ret.max(sum + t1.max(t2))
    }
}

fn main() {}
