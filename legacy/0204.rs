struct Solution;

impl Solution {
    pub fn count_primes(n: i32) -> i32 {
        let mut seen = vec![false; n as usize];
        let mut cnt = 0;

        for i in 2..n as u64 {
            if seen[i as usize] {
                continue;
            }
            cnt += 1;
            // why i * i not i * 2?
            // because when you arrive to check i
            // it means all the number i * 2, i * 3, i * (i - 1) are been checked
            // because they are divisible by 2 3 .... (i - 1)
            for j in (i * i..n as u64).step_by(i as usize) {
                seen[j as usize] = true;
            }
        }

        cnt
    }
}

fn main() {}
