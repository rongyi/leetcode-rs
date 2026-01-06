struct Solution;

impl Solution {
    pub fn maximum_beauty(mut nums: Vec<i32>, k: i32) -> i32 {
        // To solve this, we can use a sorting and sliding window approach. Since we can change any element $nums[i]$ to any value in the range $[nums[i] - k, nums[i] + k]$, two numbers $nums[i]$ and $nums[j]$ can be made equal if their transformed ranges overlap. This happens if $|nums[i] - nums[j]| \le 2k$. By sorting the array, we can find the longest subarray where the difference between the maximum and minimum elements is at most $2k$.

        nums.sort_unstable();
        let mut i = 0;
        let sz = nums.len();
        let max_diff = 2 * k;
        let mut max_win = 0;
        for j in 0..sz {
            while nums[j] - nums[i] > max_diff {
                i += 1;
            }
            max_win = max_win.max(j - i + 1);
        }

        max_win as _
    }
}

fn main() {}
