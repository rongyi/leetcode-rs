struct Solution;

impl Solution {
    pub fn can_partition(nums: Vec<i32>) -> bool {
        let sum = nums.iter().fold(0, |acc, &cur| acc + cur);
        // sum must be even
        if sum % 2 == 1 {
            return false;
        }
        let half = sum / 2;
        let max = *nums.iter().max().unwrap();
        // there's a bigger one which is large than half, we can not put this one to set
        if max > half {
            return false;
        }
        // can reach?
        let mut dp = vec![false; (half + 1) as usize];
        // acc chain start
        dp[0] = true;
        for &num in &nums {
            // 意思就是如果和可以到某个节点，那么和就可以到这个节点的值加当前节点
            // 当前节点是n，所以i - n可达，那么到n就可达了
            for i in (num..=half).rev() {
                if dp[(i - num) as usize] {
                    dp[i as usize] = true;
                }

                if dp[half as usize] {
                    return true;
                }
            }
        }

        false
    }
}

fn main() {}
