struct Solution;

impl Solution {
    // so dame hard
    // https://leetcode.com/problems/student-attendance-record-ii/discuss/101643/Share-my-O(n)-C%2B%2B-DP-solution-with-thinking-process-and-explanation
    pub fn check_record(n: i32) -> i32 {
        if n == 1 {
            return 3;
        }
        let m: i64 = 1e9 as i64 + 7;
        let n = n as usize;
        let mut a = vec![0; n + 5];
        let mut p = vec![0; n + 5];
        let mut l = vec![0; n + 5];
        p[0] = 1;
        l[0] = 1;
        l[1] = 3;
        a[0] = 1;
        a[1] = 2;
        a[2] = 4;

        // P(n) = A(n - 1) + P(n - 1) + L(n - 1), n ≥ 2.
        // L(n) = A(n - 1) + P(n - 1) + A(n - 2) + P(n - 2), n ≥ 3.
        // A(n) = A(n - 1) + A(n - 2) + A(n - 3), n ≥ 4.
        for i in 1..n {
            a[i - 1] %= m;
            p[i - 1] %= m;
            l[i - 1] %= m;

            p[i] = ((a[i - 1] + p[i - 1]) % m + l[i - 1]) % m;
            if i > 1 {
                l[i] = ((a[i - 1] + p[i - 1]) % m + (a[i - 2] + p[i - 2]) % m) % m;
            }
            if i > 2 {
                a[i] = ((a[i - 1] + a[i - 2]) % m + a[i - 3]) % m;
            }
        }

        (((a[n - 1] % m + p[n - 1] % m) % m + l[n - 1] % m) % m) as i32
    }
}

fn main() {}
