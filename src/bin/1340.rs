#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn max_jumps(arr: Vec<i32>, d: i32) -> i32 {
        let d = d as usize;
        let sz = arr.len();
        let mut dp = vec![1; sz];

        let mut ret = 0;
        for i in 0..arr.len() {
            ret = ret.max(Self::dfs(&arr, &mut dp, d, i));
        }

        ret
    }

    fn dfs(arr: &Vec<i32>, dp: &mut Vec<i32>, d: usize, i: usize) -> i32 {
        if dp[i] != 1 {
            return dp[i];
        }
        let mut max_jumps = 1;
        // jump forward
        for k in 1..=d {
            let j = i + k;
            // can only jump to lower
            if j >= arr.len() || arr[j] >= arr[i] {
                break;
            }
            max_jumps = max_jumps.max(1 + Self::dfs(arr, dp, d, j));
        }
        // jump backward
        for k in 1..=d {
            let j = i - k;
            if j >= arr.len() || arr[j] >= arr[i] {
                break;
            }
            max_jumps = max_jumps.max(1 + Self::dfs(arr, dp, d, j));
        }
        dp[i] = max_jumps;
        dp[i]
    }
}

fn main() {}
