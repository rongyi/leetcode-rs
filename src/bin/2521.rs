struct Solution;

use std::collections::HashSet;
impl Solution {
    pub fn distinct_prime_factors(nums: Vec<i32>) -> i32 {
        let mut out: HashSet<i32> = HashSet::new();

        for num in nums.into_iter() {
            Self::collect_factor(num, &mut out);
        }

        out.len() as _
    }
    fn collect_factor(mut num: i32, out: &mut HashSet<i32>) {
        for i in 2..=num {
            if num == 1 {
                break;
            }
            let mut have = false;
            while num % i == 0 {
                num /= i;
                have = true;
            }

            if have {
                out.insert(i);
            }
        }
    }
}

fn main() {}
