#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn xor_queries(arr: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let sz = arr.len();
        let mut dp = vec![vec![0; 31]; sz + 1];
        for i in 0..31 {
            for j in 0..sz {
                dp[j + 1][i] = dp[j][i];
                if (arr[j] >> i) & 1 != 0 {
                    dp[j + 1][i] += 1;
                }
            }
        }
        let mut ret = Vec::new();
        for q in queries.iter() {
            let (start, end) = (q[0] as usize, q[1] as usize);
            let mut val = 0;
            for i in 0..31 {
                let interval = dp[end + 1][i] - dp[start][i];
                if interval & 1 != 0 {
                    val |= 1 << i;
                }
            }
            ret.push(val);
        }
        ret
    }
}

fn main() {}
