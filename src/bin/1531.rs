#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn get_length_of_optimal_compression(s: String, k: i32) -> i32 {
        let s: Vec<char> = s.chars().collect();

        let mut dp = vec![vec![-1; k as usize + 1]; s.len()];

        Self::recur(&s, &mut dp, 0, k)
    }

    // at most k delete
    fn recur(s: &Vec<char>, dp: &mut Vec<Vec<i32>>, left: usize, k: i32) -> i32 {
        let sz = s.len();
        if k < 0 {
            // return very big to make this answer invalid
            return sz as i32;
        }
        // no left, or we can delete ALL left because we have enough k
        if left >= s.len() || s.len() - left <= k as usize {
            return 0;
        }
        if dp[left][k as usize] != -1 {
            return dp[left][k as usize];
        }
        let mut ret = sz as i32;
        let mut cnt = vec![0; 26];
        let mut most = 0;
        for j in left..sz {
            let idx = (s[j] as u8 - b'a') as usize;
            cnt[idx] += 1;
            most = most.max(cnt[idx]);
            ret = ret.min(
                1 + Self::digit_count(most)
                    + Self::recur(s, dp, j + 1, k - (j - left + 1 - most as usize) as i32),
            );
        }

        dp[left][k as usize] = ret;
        dp[left][k as usize]
    }

    fn digit_count(x: i32) -> i32 {
        if x == 1 {
            return 0;
        }
        if x < 10 {
            return 1;
        }
        if x < 100 {
            return 2;
        }

        3
    }
}

fn main() {}
