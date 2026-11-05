struct Solution;

impl Solution {
    pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut nums: Vec<i64> = nums.iter().map(|x| *x as i64).collect();
        nums.sort();
        let target = target as i64;
        let mut ret = Vec::new();
        let sz = nums.len();
        if sz < 4 {
            return ret;
        }
        for i in 0..sz - 3 {
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }
            for j in i + 1..sz - 2 {
                if j > i + 1 && nums[j] == nums[j - 1] {
                    continue;
                }
                let mut l = j + 1;
                let mut r = sz - 1;
                while l < r {
                    let sum = nums[i] + nums[j] + nums[l] + nums[r];
                    if sum == target {
                        ret.push(vec![
                            nums[i] as i32,
                            nums[j] as i32,
                            nums[l] as i32,
                            nums[r] as i32,
                        ]);
                        while l < r && nums[l] == nums[l + 1] {
                            l += 1;
                        }
                        while l < r && nums[r] == nums[r - 1] {
                            r -= 1;
                        }
                        l += 1;
                        r -= 1;
                    } else if sum < target {
                        l += 1;
                    } else {
                        r -= 1;
                    }
                }
            }
        }

        ret
    }
}
