struct Solution;

impl Solution {
    pub fn maximum_jumps(nums: Vec<i32>, target: i32) -> i32 {
        let sz = nums.len();
        let mut dp: Vec<i32> = vec![i32::MIN; sz];
        // already on index 0
        dp[0] = 0;

        for i in 0..sz {
            // you can not step on this index
            if dp[i] == i32::MIN {
                continue;
            }
            // let mut wayout = false;
            for j in i + 1..sz {
                if (nums[j] - nums[i]).abs() <= target {
                    dp[j] = dp[j].max(1 + dp[i]);
                }
            }
        }

        if dp[sz - 1] == i32::MIN {
            return -1;
        }
        dp[sz - 1]
    }
}

fn main() {
    let input = vec![0, 2, 1, 3];
    let val = Solution::maximum_jumps(input, 1);
    println!("{val}");
}
