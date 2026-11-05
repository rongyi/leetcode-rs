struct Solution;

impl Solution {
    pub fn nums_same_consec_diff(n: i32, k: i32) -> Vec<i32> {
        if n == 1 {
            return (0..=9).collect();
        }

        let mut ret = Vec::new();
        for i in 1..=9 {
            Self::dfs(i, n - 1, k, &mut ret);
        }

        ret
    }
    fn dfs(mut acc: i32, n: i32, k: i32, ret: &mut Vec<i32>) {
        if n == 0 {
            ret.push(acc);
            return;
        }

        let last = acc % 10;

        if last + k <= 9 {
            Self::dfs(acc * 10 + last + k, n - 1, k, ret);
        }

        if k != 0 && last - k >= 0 {
            Self::dfs(acc * 10 + last - k, n - 1, k, ret);
        }
    }
}

fn main() {}
