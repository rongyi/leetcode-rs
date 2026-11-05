struct Solution;

impl Solution {
    // https://leetcode.com/problems/k-th-smallest-in-lexicographical-order/discuss/92259/C%2B%2B-3ms-solution-using-stack
    // Test the prefix of the answer each time. If the total number of current
    // prefix is less than k, test the next prefix with current prefix +1,
    // otherwise test longer prefix with current prefix * 10
    pub fn find_kth_number(n: i32, mut k: i32) -> i32 {
        let mut cur = 1;
        while k > 0 {
            if k == 1 {
                return cur;
            }
            let sum = Self::calc(n, cur as u64);
            if k > sum {
                k -= sum;
                cur += 1;
            } else {
                k -= 1;
                cur = cur * 10;
            }
        }

        cur
    }
    fn calc(n: i32, mut x: u64) -> i32 {
        let mut ret = 0;
        let mut t = 1;
        let n = n as u64;
        while x <= n {
            if x + t - 1 <= n {
                ret += t;
            } else {
                ret += n - x + 1;
            }
            x *= 10;
            t *= 10;
        }
        ret as i32
    }
}

fn main() {}
