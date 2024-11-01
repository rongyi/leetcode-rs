struct Solution;

impl Solution {
    pub fn max_uncrossed_lines(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let m = nums1.len();
        let n = nums2.len();
        let mut dp = vec![vec![-1; n]; m];

        Self::dfs(&nums1, &nums2, 0, 0, &mut dp)
    }

    fn dfs(nums1: &Vec<i32>, nums2: &Vec<i32>, i: usize, j: usize, dp: &mut Vec<Vec<i32>>) -> i32 {
        if i >= nums1.len() || j >= nums2.len() {
            return 0;
        }
        if dp[i][j] != -1 {
            return dp[i][j];
        }
        let mut nj = j;
        while nj < nums2.len() && nums2[nj] != nums1[i] {
            nj += 1;
        }
        let cross = if nj >= nums2.len() { 0 } else { 1 };
        // 1. cross
        let v1 = cross + Self::dfs(nums1, nums2, i + 1, nj + 1, dp);
        // 2. dont cross
        let v2 = Self::dfs(nums1, nums2, i + 1, j, dp);
        dp[i][j] = v1.max(v2);

        dp[i][j]
    }
}

fn main() {}
