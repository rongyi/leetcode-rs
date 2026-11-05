struct Solution;

impl Solution {
    pub fn count_primes(n: i32) -> i32 {
        let sz = n as usize;
        // 0, 1 index is useless
        let mut is_prime = vec![true; sz];
        let mut count = 0;

        for i in 2..sz {
            if is_prime[i] {
                count += 1;
                for j in (i*2..sz).step_by(i) {
                    is_prime[j] = false;
                }
            }
        }

        count
    }
}
fn main() {
    Solution::count_primes(10);
}
