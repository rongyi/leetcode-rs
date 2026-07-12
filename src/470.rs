/// LeetCode 470: Implement Rand10() Using Rand7()
///
/// Rejection sampling: call rand7() twice → 7×7 = 49 pairs.
/// Keep 0..39 (40 values, divisible by 10), reject 40..48.
///   idx = (rand7() - 1) * 7 + (rand7() - 1)
///   if idx < 40: return idx % 10 + 1
/// Expected ~2.45 calls per rand10().

impl Solution {
    pub fn rand10() -> i32 {
        loop {
            let idx = (rand7() - 1) * 7 + (rand7() - 1);
            if idx < 40 {
                return idx % 10 + 1;
            }
        }
    }
}

struct Solution;

fn main() {}
