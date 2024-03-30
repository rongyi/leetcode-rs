struct Solution;

impl Solution {
    // https://leetcode.com/problems/non-negative-integers-without-consecutive-ones/solutions/103754/C++-Non-DP-O(32)-Fibonacci-solution/
    // The solution is based on 2 facts:

    // 1. the number of length k string without consecutive 1 is Fibonacci sequence f(k);
    // For example, if k = 5, the range is 00000-11111. We can consider it as two ranges, which are 00000-01111 and 10000-10111. Any number >= 11000 is not allowed due to consecutive 1. The first case is actually f(4), and the second case is f(3), so f(5)= f(4)+f(3).
    // Scan the number from most significant digit, i.e. left to right, in binary format. If we find a '1' with k digits to the right, count increases by f(k) because we can put a '0' at this digit and any valid length k string behind; After that, we continue the loop to consider the remaining cases, i.e., we put a '1' at this digit. If consecutive 1s are found, we exit the loop and return the answer. By the end of the loop, we return count+1 to include the number n itself.
    // For example, if n is 10010110,

    // 2. we find first '1' at 7 digits to the right, we add range 00000000-01111111, which is f(7);
    // second '1' at 4 digits to the right, add range 10000000-10001111, f(4);
    // third '1' at 2 digits to the right, add range 10010000-10010011, f(2);
    // fourth '1' at 1 digits to the right, add range 10010100-10010101, f(1);
    // Those ranges are continuous from 00000000 to 10010101. And any greater number <= n will have consecutive 1.
    pub fn find_integers(num: i32) -> i32 {
        let mut dp = vec![0; 32];
        dp[0] = 1;
        dp[1] = 2;
        for i in 2..32 {
            dp[i] = dp[i - 1] + dp[i - 2];
        }

        let mut ret = 0;
        let mut k = 30;
        let mut is_prev_one = false;
        while k >= 0 {
            if (num & (1 << k)) != 0 {
                ret += dp[k as usize];
                if is_prev_one {
                    return ret;
                }
                is_prev_one = true;
            } else {
                is_prev_one = false;
            }
            k -= 1;
        }

        // Since we have checked the consecutive 1s in the loop, and the biggest number of the range must be just smaller than n by 1, count+1 includes the n must be itself and has no consecutive. If n has consecutive 1, count must be returned by if (pre_bit) return ans; in the loop.
        ret + 1
    }
}

fn main() {}
