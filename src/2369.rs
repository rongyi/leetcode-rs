struct Solution;

impl Solution {
    pub fn valid_partition(nums: Vec<i32>) -> bool {
        let sz = nums.len();
        let mut dp = vec![false; sz + 1];
        dp[0] = true;

        for i in 1..=sz {
            // Case 1: Check for a two-element subarray
            if i >= 2 && nums[i - 1] == nums[i - 2] {
                if dp[i - 2] {
                    dp[i] = true;
                }
            }

            // Case 2 & 3: Check for a three-element subarray
            if i >= 3 {
                // Check for three equal elements
                if nums[i - 1] == nums[i - 2] && nums[i - 2] == nums[i - 3] {
                    if dp[i - 3] {
                        dp[i] = true;
                    }
                }

                // Check for three consecutive increasing elements
                if dp[i] == false
                    && nums[i - 1] == nums[i - 2] + 1
                    && nums[i - 2] == nums[i - 3] + 1
                {
                    if dp[i - 3] {
                        dp[i] = true;
                    }
                }
            }
        }
        dp[sz]
    }
    pub fn valid_partitionOneWay(nums: Vec<i32>) -> bool {
        let sz = nums.len();
        let mut memo: Vec<i32> = vec![-1; sz];
        Self::recur(&nums, 0, sz, &mut memo) == 1
    }

    fn recur(nums: &Vec<i32>, cur: usize, sz: usize, memo: &mut Vec<i32>) -> i32 {
        if cur >= sz {
            return 1;
        }
        if memo[cur] != -1 {
            return memo[cur];
        }
        // only one element in this chunk
        if cur + 1 == sz {
            memo[cur] = -1;
            return 0;
        }
        // ok, a valid split
        if nums[cur] == nums[cur + 1] {
            let rest = Self::recur(nums, cur + 2, sz, memo);
            if rest == 1 {
                memo[cur] = 1;
                return 1;
            }
        }
        if cur + 2 < sz {
            // case 1, triple three same
            if nums[cur] == nums[cur + 1] && nums[cur + 1] == nums[cur + 2] {
                let rest = Self::recur(nums, cur + 3, sz, memo);
                if rest == 1 {
                    memo[cur] = 1;
                    return 1;
                }
            }
            // case 2, consecutive number
            if nums[cur] + 1 == nums[cur + 1] && nums[cur + 1] + 1 == nums[cur + 2] {
                let rest = Self::recur(nums, cur + 3, sz, memo);
                if rest == 1 {
                    memo[cur] = 1;
                    return 1;
                }
            }
        }

        memo[cur] = 0;
        0
    }
}

fn main() {}
